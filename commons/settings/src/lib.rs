use anyhow::{bail, Result};
use device_settings::DeviceSettings;
use serde::{Deserialize, Serialize};
use std::{env, fmt, fs::File, path::PathBuf};
use tracing::{error, info};

use crate::{messaging::MessagingSettings, provisioning::ProvisioningSettings};
pub mod device_settings;
pub mod messaging;
pub mod provisioning;

/// Agent Settings - Struct corresponding to the settings.yml schema
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AgentSettings {
    pub sentry: SentrySettings,
    pub server: ServerSettings,
    pub provisioning: ProvisioningSettings,
    pub messaging: MessagingSettings,
    pub device_settings: DeviceSettings,
}

impl Default for AgentSettings {
    fn default() -> Self {
        Self {
            sentry: SentrySettings::default(),
            server: ServerSettings::default(),
            provisioning: ProvisioningSettings::default(),
            messaging: MessagingSettings::default(),
            device_settings: DeviceSettings::default(),
        }
    }
}

/// SentrySettings - Settings parameter for configuring the sentry client
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SentrySettings {
    pub enabled: bool,
    pub dsn: Option<String>,
}

impl Default for SentrySettings {
    fn default() -> Self {
        Self {
            enabled: false,
            dsn: None,
        }
    }
}

/// ServerSettings - Settings parameter for configuring the agent's grpc server settings
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ServerSettings {
    pub url: Option<String>,
    pub port: i16,
    pub tls: bool,
}

impl Default for ServerSettings {
    fn default() -> Self {
        Self {
            url: Some(String::from("127.0.0.1")),
            port: 3001,
            tls: false,
        }
    }
}

/// # Agent Error Codes
///
/// Implements standard errors for the agent
#[derive(Debug, Default, Clone, Copy)]
pub enum SettingsErrorCodes {
    #[default]
    UnknownError,
    ReadError,
    ParseError,
}

impl fmt::Display for SettingsErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SettingsErrorCodes::UnknownError => write!(f, "SettingsErrorCodes: UnknownError"),
            SettingsErrorCodes::ReadError => write!(f, "SettingsErrorCodes: ReadError"),
            SettingsErrorCodes::ParseError => write!(f, "SettingsErrorCodes: ParseError"),
        }
    }
}

/// # SettingsError
///
/// Implements a standard error type for all agent related errors
/// includes the error code (`SettingsErrorCodes`) and a message
#[derive(Debug, Default)]
pub struct SettingsError {
    pub code: SettingsErrorCodes,
    pub message: String,
}

impl SettingsError {
    pub fn new(code: SettingsErrorCodes, message: String) -> Self {
        error!("Error: (code: {:?}, message: {})", code, message);
        Self {
            code,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for SettingsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}

/// # Reads Settings path from arg
///
/// Reads the `-s` or `--settings` argument for the path
pub fn read_settings_path_from_args() -> Option<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 && (args[1] == "-s" || args[1] == "--settings") {
        info!("using settings path from argument - {}", args[2]);
        return Some(String::from(args[2].clone()));
    }
    None
}

/// # Reads Settings YML
///
/// Reads the `settings.yml` and parsers to AgentSettings
///
/// **Important**: Ensure all fields are present in the yml due to strict parsing
pub fn read_settings_yml() -> Result<AgentSettings> {
    // Add schema validator for yml
    let mut file_path = PathBuf::from(
        std::env::var("MECHA_AGENT_SETTINGS_PATH")
            .unwrap_or(String::from("~/.mecha/agent/settings.yml")),
    ); // Get path of the library

    // TODO: handle semver version support

    // read from args
    let file_path_in_args = read_settings_path_from_args();
    if file_path_in_args.is_some() {
        file_path = PathBuf::from(file_path_in_args.unwrap());
    }

    info!(
        task = "read_settings_yml",
        "reading settings from file location - {:?}", file_path
    );

    // open file
    let settings_file_handle = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            // TODO: add capture
            bail!(SettingsError::new(
                SettingsErrorCodes::ReadError,
                format!(
                    "error read the settings.yml in the path - {}",
                    e.to_string()
                ),
            ));
        }
    };

    // read and parse
    let config: AgentSettings = match serde_yaml::from_reader(settings_file_handle) {
        Ok(config) => config,
        Err(e) => {
            bail!(SettingsError::new(
                SettingsErrorCodes::ParseError,
                format!("error parsing the settings.yml - {}", e.to_string()),
            ));
        }
    };

    Ok(config)
}