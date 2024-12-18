use std::fmt;

#[derive(Debug, Default, Clone, Copy)]
pub enum DeviceSettingErrorCodes {
    #[default]
    UnknownError,
    ExtractAddTaskPayloadError,
    MessageHeaderEmptyError,
    AckHeaderNotFoundError,
    PullMessagesError,
    CreateConsumerError,
    ChannelSendMessageError,
    ChannelReceiveMessageError,
    SettingsDatabaseDeleteError,
}

impl fmt::Display for DeviceSettingErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DeviceSettingErrorCodes::UnknownError => {
                write!(f, "DeviceSettingErrorCodes: UnknownError")
            }
            DeviceSettingErrorCodes::ExtractAddTaskPayloadError => {
                write!(f, "DeviceSettingErrorCodes: ExtractAddTaskPayloadError")
            }
            DeviceSettingErrorCodes::AckHeaderNotFoundError => {
                write!(f, "DeviceSettingErrorCodes: AckHeaderNotFoundError")
            }
            DeviceSettingErrorCodes::MessageHeaderEmptyError => {
                write!(f, "DeviceSettingErrorCodes: MessageHeaderEmptyError")
            }
            DeviceSettingErrorCodes::PullMessagesError => {
                write!(f, "DeviceSettingErrorCodes: PullMessagesError")
            }
            DeviceSettingErrorCodes::CreateConsumerError => {
                write!(f, "DeviceSettingErrorCodes: CreateConsumerError")
            }
            DeviceSettingErrorCodes::ChannelSendMessageError => {
                write!(f, "DeviceSettingErrorCodes: ChannelSendMessageError",)
            }
            DeviceSettingErrorCodes::ChannelReceiveMessageError => {
                write!(f, "DeviceSettingErrorCodes: ChannelReceiveMessageError")
            }
            DeviceSettingErrorCodes::SettingsDatabaseDeleteError => {
                write!(f, "DeviceSettingErrorCodes: SettingsDatabaseDeleteError")
            }
        }
    }
}

#[derive(Debug)]
pub struct DeviceSettingError {
    pub code: DeviceSettingErrorCodes,
    pub message: String,
}

impl std::fmt::Display for DeviceSettingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DeviceSettingErrorCodes:(code: {:?}, message: {})",
            self.code, self.message
        )
    }
}

impl DeviceSettingError {
    pub fn new(code: DeviceSettingErrorCodes, message: String) -> Self {
        Self { code, message }
    }
}
