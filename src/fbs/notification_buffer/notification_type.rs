#[derive(Clone, Debug, PartialEq)]
pub enum NotificationType {
    ///
    /// An action triggered an alert.
    ///
    /// An alert is a internal warning that has been managed by the system but
    /// require user attention because it may be the result of a misusage.
    ///
    Alert,

    ///
    /// An action triggered an error.
    ///
    /// An error is a critical failure of an instance driver.
    /// The given driver is not able to process any more data and will try to reboot.
    ///
    Error,
}

impl From<NotificationType> for u16 {
    fn from(notification_type: NotificationType) -> Self {
        match notification_type {
            NotificationType::Alert => 1,
            NotificationType::Error => 2,
        }
    }
}

impl TryFrom<u16> for NotificationType {
    type Error = String;

    fn try_from(value: u16) -> Result<Self, <Self as TryFrom<u16>>::Error> {
        match value {
            1 => Ok(NotificationType::Alert),
            2 => Ok(NotificationType::Error),
            _ => Err(format!("Invalid NotificationType value: {}", value)),
        }
    }
}
