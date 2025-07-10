pub mod notification_type;
use crate::fbs::{
    panduza_generated::panduza::{Message, Notification},
    PzaBuffer,
};
use bytes::Bytes;
pub use notification_type::NotificationType;
use zenoh::bytes::ZBytes;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct NotificationBuffer {
    raw_data: Bytes,
}

impl PzaBuffer for NotificationBuffer {
    fn from_zbytes(zbytes: ZBytes) -> Self {
        let bytes = Bytes::copy_from_slice(&zbytes.to_bytes());
        NotificationBuffer { raw_data: bytes }
    }

    fn to_zbytes(self) -> ZBytes {
        ZBytes::from(self.raw_data)
    }

    fn source(&self) -> u16 {
        let msg = self.as_message();
        msg.header().map(|h| h.source()).unwrap_or(0)
    }

    fn sequence(&self) -> u16 {
        let msg = self.as_message();
        msg.header().map(|h| h.sequence()).unwrap_or(0)
    }

    fn as_message(&self) -> Message {
        flatbuffers::root::<Message>(&self.raw_data)
            .expect("Failed to deserialize Message from raw_data")
    }

    fn has_value_equal_to_message_value(&self, _message: &Message) -> bool {
        // NotificationBuffer does not have a value field to compare
        true
    }
}

impl NotificationBuffer {
    /// Returns the Notification object from the buffer, if present.
    pub fn notification(&self) -> Option<Notification> {
        self.as_message().payload_as_notification()
    }

    /// Returns the notification type, if present.
    pub fn notification_type(&self) -> Option<NotificationType> {
        self.notification()
            .and_then(|n| NotificationType::try_from(n.type_()).ok())
    }

    /// Returns the source string, if present.
    pub fn source_str(&self) -> Option<&str> {
        self.notification().and_then(|n| n.source())
    }

    /// Returns the message string, if present.
    pub fn message_str(&self) -> Option<&str> {
        self.notification().and_then(|n| n.message())
    }
}
