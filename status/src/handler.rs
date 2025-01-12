use agent_settings::status::StatusSettings;
use anyhow::{bail, Result};
use events::Event;
use identity::handler::IdentityMessage;
use messaging::handler::MessagingMessage;
use tokio::{
    select,
    sync::{
        broadcast,
        mpsc::{self, Sender},
        oneshot,
    },
    task::JoinHandle,
};
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

use crate::{
    errors::{StatusError, StatusErrorCodes},
    service::{machine_platform_info, send_status, SendStatusOptions},
};

pub struct StatusHandler {
    event_tx: broadcast::Sender<Event>,
    messaging_tx: Sender<MessagingMessage>,
    identity_tx: Sender<IdentityMessage>,
    timer_token: Option<CancellationToken>,
    settings: StatusSettings,
}

pub enum StatusMessage {
    Send {
        reply_to: oneshot::Sender<Result<bool>>,
    },
}
pub struct StatusOptions {
    pub event_tx: broadcast::Sender<Event>,
    pub messaging_tx: Sender<MessagingMessage>,
    pub identity_tx: Sender<IdentityMessage>,
    pub settings: StatusSettings,
}

impl StatusHandler {
    pub fn new(options: StatusOptions) -> Self {
        Self {
            event_tx: options.event_tx,
            messaging_tx: options.messaging_tx,
            identity_tx: options.identity_tx,
            timer_token: None,
            settings: options.settings,
        }
    }
    pub async fn set_timer(&mut self) -> Result<()> {
        info!(func = "set_timer", package = env!("CARGO_PKG_NAME"), "init");

        // safety: check for existing cancel token, and cancel it
        let exist_timer_token = &self.timer_token;
        if exist_timer_token.is_some() {
            let _ = exist_timer_token.as_ref().unwrap().cancel();
        }

        // create a new token
        let timer_token = CancellationToken::new();
        let timer_token_cloned = timer_token.clone();
        let messaging_tx = self.messaging_tx.clone();
        let identity_tx = self.identity_tx.clone();
        let interval = self.settings.interval;
        // create spawn for timer
        let _: JoinHandle<Result<()>> = tokio::task::spawn(async move {
            let mut timer = tokio::time::interval(std::time::Duration::from_secs(interval));
            loop {
                select! {
                        _ = timer_token.cancelled() => {
                            return Ok(());
                        },
                        _ = timer.tick() => {
                            let _ = send_status(SendStatusOptions {
                                messaging_tx: messaging_tx.clone(),
                                identity_tx: identity_tx.clone(),
                            }).await;
                    }
                }
            }
        });

        // Save to state
        self.timer_token = Some(timer_token_cloned);

        Ok(())
    }

    pub fn clear_timer(&self) -> Result<bool> {
        let exist_timer_token = &self.timer_token;
        if exist_timer_token.is_some() {
            let _ = exist_timer_token.as_ref().unwrap().cancel();
        } else {
            return Ok(false);
        }
        Ok(true)
    }

    pub async fn run(&mut self, mut message_rx: mpsc::Receiver<StatusMessage>) -> Result<()> {
        info!(
            func = "run",
            package = env!("CARGO_PKG_NAME"),
            "status service initiated"
        );
        let mut event_rx = self.event_tx.subscribe();
        loop {
            select! {
                    msg = message_rx.recv() => {
                        if msg.is_none() {
                            error!(
                                func = "run",
                                package = env!("CARGO_PKG_NAME"),
                                "error receiving message"
                            );
                            bail!(StatusError::new(
                                StatusErrorCodes::ChannelReceiveMessageError,
                                "failed to receive message".to_string(),
                            ));
                        }

                        match msg.unwrap() {
                            StatusMessage::Send { reply_to } => {
                                let res = send_status(SendStatusOptions {
                                    messaging_tx: self.messaging_tx.clone(),
                                    identity_tx: self.identity_tx.clone(),
                                }).await;
                                let _ = reply_to.send(res);
                            }
                        };
                    }
                    // Receive events from other services
                    event = event_rx.recv() => {
                        if event.is_err() {
                            continue;
                        }
                        match event.unwrap() {
                            Event::Messaging(events::MessagingEvent::Connected) => {
                                // start
                                let _ = machine_platform_info(self.identity_tx.clone(), self.messaging_tx.clone()).await;
                                let _ = &self.set_timer().await;
                            },
                            Event::Messaging(events::MessagingEvent::Disconnected) => {
                                let _ = &self.clear_timer();
                            },
                            Event::Provisioning(events::ProvisioningEvent::Deprovisioned) => {
                                let _ = &self.clear_timer();
                            },
                            _ => {},
                        }
                    }
            }
        }
    }
}
