use super::generic::PanduzaBuffer;
use super::panduza_generated::panduza::{
    Header, HeaderArgs, Message, MessageArgs, Notification, NotificationArgs, Payload,
};
use bytes::Bytes;
use rand::Rng;
use zenoh::bytes::ZBytes;

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

// ----------------------------------------------------------------------------

///
///
#[derive(Default, Clone, Debug, PartialEq)]
pub struct NotificationBuffer {
    ///
    ///
    pub notification_type: Option<NotificationType>,

    ///
    ///
    pub source: Option<String>,

    ///
    ///
    pub message: Option<String>,

    ///
    ///
    pub sequence: Option<u16>,

    /// Internal Raw Data
    ///
    pub raw_data: Option<Bytes>,
}

impl NotificationBuffer {
    ///
    ///
    pub fn new() -> Self {
        Self::default()
    }

    ///
    ///
    pub fn with_type(mut self, notification_type: NotificationType) -> Self {
        self.notification_type = Some(notification_type);
        self
    }

    ///
    ///
    pub fn with_source<T: Into<String>>(mut self, source: T) -> Self {
        self.source = Some(source.into());
        self
    }

    ///
    ///
    pub fn with_message<T: Into<String>>(mut self, message: T) -> Self {
        self.message = Some(message.into());
        self
    }

    ///
    ///
    pub fn is_builded(&self) -> bool {
        self.raw_data.is_some()
    }

    ///
    ///
    pub fn sequence(&self) -> u16 {
        self.sequence
            .expect("Sequence must be set before accessing it")
    }

    ///
    ///
    pub fn to_zbytes(self) -> ZBytes {
        ZBytes::from(
            self.raw_data
                .expect("Raw data must be set before converting to ZBytes"),
        )
    }

    ///
    ///
    pub fn as_message(&self) -> Message {
        let data = self
            .raw_data
            .as_ref()
            .expect("Buffer must be built to access the message");
        flatbuffers::root::<Message>(data).expect("Failed to deserialize Message from raw_data")
    }

    ///
    ///
    pub fn notification(&self) -> Option<Notification> {
        self.as_message().payload_as_notification()
    }

    ///
    ///
    pub fn notification_type(&self) -> Option<NotificationType> {
        self.notification()
            .and_then(|n| NotificationType::try_from(n.type_()).ok())
    }

    ///
    ///
    pub fn source_str(&self) -> Option<&str> {
        self.notification().and_then(|n| n.source())
    }

    ///
    ///
    pub fn message_str(&self) -> Option<&str> {
        self.notification().and_then(|n| n.message())
    }
}

impl PanduzaBuffer for NotificationBuffer {
    fn new() -> Self {
        Self::default()
    }

    fn with_value<T>(self, _value: T) -> Self
    where
        T: Into<Self>,
    {
        // NotificationBuffer does not use a generic value, so just return self
        self
    }

    fn with_source(self, source: u16) -> Self {
        // For NotificationBuffer, source is a String, so convert u16 to String
        Self {
            source: Some(source.to_string()),
            ..self
        }
    }

    fn with_sequence(self, sequence: u16) -> Self {
        Self {
            sequence: Some(sequence),
            ..self
        }
    }

    fn with_random_sequence(self) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            sequence: Some(rng.gen::<u16>()),
            ..self
        }
    }

    ///
    ///
    fn build(self) -> Result<Self, String> {
        let mut builder = flatbuffers::FlatBufferBuilder::new();

        let source = self.source.as_ref().map(|s| builder.create_string(s));
        let message = self.message.as_ref().map(|m| builder.create_string(m));

        // Generate timestamp for header only
        let timestamp = super::common::generate_timestamp();

        let notification_args = NotificationArgs {
            type_: self
                .notification_type
                .clone()
                .map(|t| t.into())
                .ok_or("Notification type not provided")?,
            source,
            message,
        };
        let notification = Notification::create(&mut builder, &notification_args);

        let sequence = self.sequence.ok_or("Sequence not provided")?;
        let header_args = HeaderArgs {
            timestamp: Some(&timestamp),
            source: 0, // Not used for notification, set to 0
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

        Ok(Self {
            raw_data: Some(Bytes::from(builder.finished_data().to_vec())),
            notification_type: self.notification_type,
            source: self.source,
            message: self.message,
            sequence: self.sequence,
        })
    }

    ///
    ///
    fn build_from_zbytes(zbytes: ZBytes) -> Self {
        let bytes = Bytes::copy_from_slice(&zbytes.to_bytes());
        Self {
            raw_data: Some(bytes),
            notification_type: None,
            source: None,
            message: None,
            sequence: None,
        }
    }

    fn is_builded(&self) -> bool {
        self.is_builded()
    }

    fn sequence(&self) -> u16 {
        self.sequence()
    }

    fn to_zbytes(self) -> ZBytes {
        self.to_zbytes()
    }

    fn as_message(&self) -> Message {
        self.as_message()
    }

    fn has_value_equal_to_message_value(&self, _message: &Message) -> bool {
        // NotificationBuffer does not have a value field to compare
        true
    }
}
