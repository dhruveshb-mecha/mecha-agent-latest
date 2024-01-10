use sentry_anyhow::capture_anyhow;
use std::fmt;

#[derive(Debug, Default, Clone, Copy)]
pub enum AgentErrorCodes {
    #[default]
    UnknownError,
    ProvisioningInitError,
    IdentityInitError,
    MessagingInitError,
    HeartbeatInitError,
    SettingsInitError,
    NetworkingInitError,
    TelemetryInitError,
    InitGRPCError,
}

impl fmt::Display for AgentErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AgentErrorCodes::UnknownError => write!(f, "UnknownError"),
            AgentErrorCodes::ProvisioningInitError => write!(f, "ProvisioningInitError"),
            AgentErrorCodes::IdentityInitError => write!(f, "IdentityInitError"),
            AgentErrorCodes::MessagingInitError => write!(f, "MessagingInitError"),
            AgentErrorCodes::HeartbeatInitError => write!(f, "HeartbeatInitError"),
            AgentErrorCodes::SettingsInitError => write!(f, "SettingsInitError"),
            AgentErrorCodes::NetworkingInitError => write!(f, "NetworkingInitError"),
            AgentErrorCodes::TelemetryInitError => write!(f, "TelemetryInitError"),
            AgentErrorCodes::InitGRPCError => write!(f, "InitGRPCError"),
        }
    }
}

#[derive(Debug)]
pub struct AgentError {
    pub code: AgentErrorCodes,
    pub message: String,
}

impl std::fmt::Display for AgentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AgentErrorCodes:(code: {:?}, message: {})",
            self.code, self.message
        )
    }
}

impl AgentError {
    pub fn new(code: AgentErrorCodes, message: String, capture_error: bool) -> Self {
        if capture_error {
            let error = &anyhow::anyhow!(code)
                .context(format!("error: (code: {:?}, message: {})", code, message));
            capture_anyhow(error);
        }
        Self { code, message }
    }
}