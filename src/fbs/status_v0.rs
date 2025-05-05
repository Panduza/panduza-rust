#[allow(warnings)]
pub mod status_v0_generated;

use bytes::Bytes;
use status_v0_generated::{Notification, NotificationArgs, Timestamp};
use std::time::{SystemTime, UNIX_EPOCH};

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

#[derive(Debug)]
///
///
pub struct NotificationBuffer {
    /// Internal Raw Data
    ///
    raw_data: Bytes,
}

impl NotificationBuffer {
    ///
    ///
    pub fn from_raw_data(raw_data: Bytes) -> Self {
        Self { raw_data: raw_data }
    }

    ///
    ///
    pub fn raw_data(&self) -> &Bytes {
        &self.raw_data
    }

    ///
    ///
    pub fn take_data(self) -> Bytes {
        self.raw_data
    }

    ///
    ///
    fn generate_timestamp() -> Timestamp {
        let now = SystemTime::now();
        let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
        let seconds = since_the_epoch.as_secs();
        let nanoseconds = since_the_epoch.subsec_nanos();
        Timestamp::new(seconds, nanoseconds)
    }

    ///
    ///
    pub fn from_args(r#type: NotificationType, source: String, message: String) -> Self {
        let mut builder = flatbuffers::FlatBufferBuilder::new();

        let source = builder.create_string(&source);
        let message = builder.create_string(&message);

        let timestamp = Self::generate_timestamp();

        let object = Notification::create(
            &mut builder,
            &NotificationArgs {
                type_: r#type.into(),
                source: Some(source),
                message: Some(message),
                timestamp: Some(&timestamp),
            },
        );

        builder.finish(object, None);

        let raw_data = Bytes::from(builder.finished_data().to_vec());

        // Here we copy into the buffer
        Self { raw_data: raw_data }
    }

    ///
    ///
    pub fn object(&self) -> Notification {
        flatbuffers::root::<Notification>(&self.raw_data).unwrap()
    }
}
