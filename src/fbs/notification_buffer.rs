pub mod notification_type;
use crate::fbs::{
    panduza_generated::panduza::{Message, Notification},
    PzaBuffer,
};
use bytes::Bytes;
pub use notification_type::NotificationType;
use zenoh::bytes::ZBytes;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct NotificationBufferBuilder {
    notification_type: Option<NotificationType>,
    source: Option<String>,
    message: Option<String>,
    sequence: Option<u16>,
    header_source: Option<u16>,
}

impl NotificationBufferBuilder {
    pub fn with_notification_type(mut self, notification_type: NotificationType) -> Self {
        self.notification_type = Some(notification_type);
        self
    }

    pub fn with_notification_source<S: Into<String>>(mut self, source: S) -> Self {
        self.source = Some(source.into());
        self
    }

    pub fn with_notification_message<S: Into<String>>(mut self, message: S) -> Self {
        self.message = Some(message.into());
        self
    }

    pub fn with_sequence(mut self, sequence: u16) -> Self {
        self.sequence = Some(sequence);
        self
    }

    pub fn with_header_source(mut self, header_source: u16) -> Self {
        self.header_source = Some(header_source);
        self
    }

    pub fn build(self) -> Result<NotificationBuffer, String> {
        use crate::fbs::common::generate_timestamp;
        use crate::fbs::panduza_generated::panduza::{
            Header, HeaderArgs, Message, MessageArgs, Notification as FbNotification,
            NotificationArgs, Payload,
        };
        let mut builder = flatbuffers::FlatBufferBuilder::new();
        let timestamp = generate_timestamp();

        let notification_type = self
            .notification_type
            .ok_or("notification_type not provided".to_string())?;
        let source_str = self.source.as_deref().unwrap_or("");
        let message_str = self.message.as_deref().unwrap_or("");

        let source_fb = builder.create_string(source_str);
        let message_fb = builder.create_string(message_str);

        let notification_args = NotificationArgs {
            type_: notification_type as u16,
            source: Some(source_fb),
            message: Some(message_fb),
        };
        let notification = FbNotification::create(&mut builder, &notification_args);

        let header_source = self
            .header_source
            .ok_or("header_source not provided".to_string())?;
        let sequence = self.sequence.ok_or("sequence not provided".to_string())?;

        let header_args = HeaderArgs {
            timestamp: Some(&timestamp),
            source: header_source,
            sequence,
        };
        let header = Header::create(&mut builder, &header_args);

        let message_args = MessageArgs {
            header: Some(header),
            payload_type: Payload::Notification,
            payload: Some(notification.as_union_value()),
        };
        let message = Message::create(&mut builder, &message_args);

        builder.finish(message, None);

        Ok(NotificationBuffer {
            raw_data: Bytes::from(builder.finished_data().to_vec()),
        })
    }
}

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
