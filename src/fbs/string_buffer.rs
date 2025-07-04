use super::common::generate_timestamp;
use super::generic::PanduzaBuffer;
use super::panduza_generated::panduza::{
    Header, HeaderArgs, Message, MessageArgs, Payload, String as FbString, StringArgs,
};
use bytes::Bytes;
use rand::Rng;
use zenoh::bytes::ZBytes;

#[derive(Default, Clone, Debug, PartialEq)]
/// StringBuffer is a wrapper around a flatbuffer serialized Message with a String payload.
/// It provides methods to create, access, and manipulate string data.
pub struct StringBuffer {
    ///
    value: Option<String>,

    ///
    whitelist: Option<Vec<String>>,

    ///
    source: Option<u16>,

    ///
    sequence: Option<u16>,

    ///
    raw_data: Option<Bytes>,
}

/// Implementation of GenericBuffer for StringBuffer
///
impl PanduzaBuffer for StringBuffer {
    fn new() -> Self {
        Self {
            value: None,
            whitelist: None,
            source: None,
            sequence: None,
            raw_data: None,
        }
    }

    fn with_value<T>(self, value: T) -> Self
    where
        T: Into<Self>,
    {
        Self {
            value: value.into().value,
            ..self
        }
    }

    fn with_source(self, source: u16) -> Self {
        Self {
            source: Some(source),
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

    fn build(self) -> Result<Self, String> {
        let mut builder = flatbuffers::FlatBufferBuilder::new();

        // Create the value string
        let value_str = self.value.as_ref().ok_or("value not provided")?;
        let value_offset = builder.create_string(value_str);

        // Create whitelist vector if specified
        let whitelist = if let Some(ref wl) = self.whitelist {
            let whitelist_strings: Vec<_> = wl
                .iter()
                .map(|s| builder.create_string(s))
                .collect();
            Some(builder.create_vector(&whitelist_strings))
        } else {
            None
        };

        // Create the string payload
        let string_args = StringArgs {
            value: Some(value_offset),
            whitelist,
        };
        let string = FbString::create(&mut builder, &string_args);

        // Create header with timestamp
        let timestamp = generate_timestamp();
        let source = self.source.ok_or("source not provided".to_string())?;
        let sequence = self.sequence.ok_or("sequence not provided".to_string())?;

        let header_args = HeaderArgs {
            timestamp: Some(&timestamp),
            source,
            sequence,
        };
        let header = Header::create(&mut builder, &header_args);

        // Create the message with the string payload
        let message_args = MessageArgs {
            header: Some(header),
            payload_type: Payload::String,
            payload: Some(string.as_union_value()),
        };
        let message = Message::create(&mut builder, &message_args);

        builder.finish(message, None);

        Ok(Self {
            raw_data: Some(Bytes::from(builder.finished_data().to_vec())),
            value: self.value,
            whitelist: self.whitelist,
            source: self.source,
            sequence: self.sequence,
        })
    }

    fn build_from_zbytes(zbytes: ZBytes) -> Self {
        let bytes = Bytes::copy_from_slice(&zbytes.to_bytes());
        Self {
            raw_data: Some(bytes),
            value: None,
            whitelist: None,
            source: None,
            sequence: None,
        }
    }

    ///
    fn is_builded(&self) -> bool {
        self.raw_data.is_some()
    }

    ///
    fn sequence(&self) -> u16 {
        self.sequence
            .expect("Sequence must be set before accessing it")
    }

    fn to_zbytes(self) -> ZBytes {
        ZBytes::from(
            self.raw_data
                .expect("Raw data must be set before converting to ZBytes"),
        )
    }

    fn as_message(&self) -> Message {
        let data = self
            .raw_data
            .as_ref()
            .expect("Buffer must be built to access the message");
        flatbuffers::root::<Message>(data).expect("Failed to deserialize Message from raw_data")
    }

    ///
    ///
    fn has_value_equal_to_message_value(&self, message: &Message) -> bool {
        if let Some(payload) = message.payload_as_string() {
            if let Some(ref value) = self.value {
                if let Some(payload_value) = payload.value() {
                    return payload_value == value;
                }
            }
        }
        false
    }
}

impl From<String> for StringBuffer {
    fn from(value: String) -> Self {
        let mut obj = Self::new();
        obj.value = Some(value);
        obj
    }
}

impl From<&str> for StringBuffer {
    fn from(value: &str) -> Self {
        let mut obj = Self::new();
        obj.value = Some(value.to_string());
        obj
    }
}

impl From<StringBuffer> for String {
    fn from(buffer: StringBuffer) -> Self {
        match buffer.value {
            Some(v) => v,
            None => buffer.value(),
        }
    }
}

impl StringBuffer {
    /// Extracts the String payload from the Message
    ///
    /// # Returns
    /// The deserialized String object, or None if the payload is not a String
    pub fn string(&self) -> Option<FbString> {
        self.as_message().payload_as_string()
    }

    /// Gets the string value from the payload
    ///
    /// # Returns
    /// The string value, or an empty string if the payload is not a valid String
    pub fn value(&self) -> String {
        self.string()
            .and_then(|s| s.value())
            .map(|s| s.to_string())
            .unwrap_or_default()
    }

    /// Sets the whitelist of allowed values
    pub fn with_whitelist(self, whitelist: Vec<String>) -> Self {
        Self {
            whitelist: Some(whitelist),
            ..self
        }
    }

    /// Gets the whitelist from the payload
    ///
    /// # Returns
    /// The whitelist of allowed values, or None if no whitelist is set
    pub fn whitelist(&self) -> Option<Vec<String>> {
        None
    }
}
