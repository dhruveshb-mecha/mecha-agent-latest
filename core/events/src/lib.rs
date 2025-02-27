use async_nats::Event as NatsEvent;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum ProvisioningEvent {
    Provisioned,
    Deprovisioned,
}

#[derive(Debug, Clone)]
pub enum MessagingEvent {
    Connected,
    Disconnected,
    Reconnected,
}

#[derive(Debug, Clone)]
pub enum SettingEvent {
    Synced,
    Updated {
        existing_settings: HashMap<String, String>,
        new_settings: HashMap<String, String>,
    },
}

#[derive(Debug, Clone)]
pub enum Event {
    Provisioning(ProvisioningEvent),
    Messaging(MessagingEvent),
    Settings(SettingEvent),
    Nats(NatsEvent),
}
