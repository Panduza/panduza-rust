use super::common::generate_timestamp;
use super::generic::PanduzaBuffer;
use super::panduza_generated::panduza::{
    Header, HeaderArgs, Message, MessageArgs, Number, NumberArgs, NumberRange, NumberRangeArgs,
    Payload, SIPrefix, SIUnit, Unit, UnitArgs,
};
use bytes::Bytes;
use rand::Rng;
use zenoh::bytes::ZBytes;

#[derive(Default, Clone, Debug, PartialEq)]
/// NumberBuffer is a wrapper around a flatbuffer serialized Message with a Number payload.
/// It provides methods to create, access, and manipulate numeric data.
pub struct NumberBuffer {
    ///
    value: Option<f64>,

    ///
    unit_prefix: Option<SIPrefix>,

    ///
    unit_type: Option<SIUnit>,

    ///
    decimals: Option<u8>,

    ///
    range_min: Option<f64>,

    ///
    range_max: Option<f64>,

    ///
    whitelist: Option<Vec<f64>>,

    ///
    source: Option<u16>,

    ///
    sequence: Option<u16>,

    ///
    raw_data: Option<Bytes>,
}

/// Implementation of GenericBuffer for NumberBuffer
///
impl PanduzaBuffer for NumberBuffer {
    fn new() -> Self {
        Self {
            value: None,
            unit_prefix: None,
            unit_type: None,
            decimals: None,
            range_min: None,
            range_max: None,
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

        // Create Unit if specified
        let unit = if self.unit_prefix.is_some() || self.unit_type.is_some() {
            let unit_args = UnitArgs {
                prefix: self.unit_prefix.unwrap_or(SIPrefix::NONE),
                unit: self.unit_type.unwrap_or(SIUnit::NONE),
            };
            Some(Unit::create(&mut builder, &unit_args))
        } else {
            None
        };

        // Create NumberRange if specified
        let range = if self.range_min.is_some() || self.range_max.is_some() {
            let range_args = NumberRangeArgs {
                min: self.range_min.unwrap_or(0.0),
                max: self.range_max.unwrap_or(0.0),
            };
            Some(NumberRange::create(&mut builder, &range_args))
        } else {
            None
        };

        // Create whitelist vector if specified
        let whitelist = if let Some(ref wl) = self.whitelist {
            Some(builder.create_vector(wl))
        } else {
            None
        };

        // Create the number payload
        let number_args = NumberArgs {
            value: self.value.ok_or("value not provided".to_string())?,
            unit,
            decimals: self.decimals.unwrap_or(0),
            range,
            whitelist,
        };
        let number = Number::create(&mut builder, &number_args);

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

        // Create the message with the number payload
        let message_args = MessageArgs {
            header: Some(header),
            payload_type: Payload::Number,
            payload: Some(number.as_union_value()),
        };
        let message = Message::create(&mut builder, &message_args);

        builder.finish(message, None);

        Ok(Self {
            raw_data: Some(Bytes::from(builder.finished_data().to_vec())),
            value: self.value,
            unit_prefix: self.unit_prefix,
            unit_type: self.unit_type,
            decimals: self.decimals,
            range_min: self.range_min,
            range_max: self.range_max,
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
            unit_prefix: None,
            unit_type: None,
            decimals: None,
            range_min: None,
            range_max: None,
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
        if let Some(payload) = message.payload_as_number() {
            if let Some(value) = self.value {
                return (payload.value() - value).abs() < f64::EPSILON;
            }
        }
        false
    }
}

impl From<f64> for NumberBuffer {
    fn from(value: f64) -> Self {
        let mut obj = Self::new();
        obj.value = Some(value);
        obj
    }
}

impl From<NumberBuffer> for f64 {
    fn from(buffer: NumberBuffer) -> Self {
        match buffer.value {
            Some(v) => v,
            None => buffer.value(),
        }
    }
}

impl NumberBuffer {
    /// Extracts the Number payload from the Message
    ///
    /// # Returns
    /// The deserialized Number object, or None if the payload is not a Number
    pub fn number(&self) -> Option<Number> {
        self.as_message().payload_as_number()
    }

    /// Gets the numeric value from the payload
    ///
    /// # Returns
    /// The numeric value, or 0.0 if the payload is not a valid Number
    pub fn value(&self) -> f64 {
        self.number().map_or(0.0, |n| n.value())
    }

    /// Sets the unit for this number
    pub fn with_unit(self, prefix: SIPrefix, unit: SIUnit) -> Self {
        Self {
            unit_prefix: Some(prefix),
            unit_type: Some(unit),
            ..self
        }
    }

    /// Sets the number of decimal places
    pub fn with_decimals(self, decimals: u8) -> Self {
        Self {
            decimals: Some(decimals),
            ..self
        }
    }

    /// Sets the range for this number
    pub fn with_range(self, min: f64, max: f64) -> Self {
        Self {
            range_min: Some(min),
            range_max: Some(max),
            ..self
        }
    }

    /// Sets the whitelist of allowed values
    pub fn with_whitelist(self, whitelist: Vec<f64>) -> Self {
        Self {
            whitelist: Some(whitelist),
            ..self
        }
    }

    /// Gets the unit from the payload
    ///
    /// # Returns
    /// The unit, or None if no unit is set
    pub fn unit(&self) -> Option<(SIPrefix, SIUnit)> {
        self.number()
            .and_then(|n| n.unit().map(|u| (u.prefix(), u.unit())))
    }

    /// Gets the number of decimals from the payload
    ///
    /// # Returns
    /// The number of decimals, or 0 if not set
    pub fn decimals(&self) -> u8 {
        self.number().map_or(0, |n| n.decimals())
    }

    /// Gets the range from the payload
    ///
    /// # Returns
    /// The range as (min, max), or None if no range is set
    pub fn range(&self) -> Option<(f64, f64)> {
        self.number()
            .and_then(|n| n.range().map(|r| (r.min(), r.max())))
    }

    /// Gets the whitelist from the payload
    ///
    /// # Returns
    /// The whitelist of allowed values, or None if no whitelist is set
    pub fn whitelist(&self) -> Option<Vec<f64>> {
        self.number().and_then(|n| {
            n.whitelist()
                .map(|wl| (0..wl.len()).map(|i| wl.get(i)).collect())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fbs::panduza_generated::panduza::{SIPrefix, SIUnit};

    #[test]
    fn test_number_buffer_creation() {
        let buffer = NumberBuffer::new()
            .with_value(42.5)
            .with_source(1)
            .with_sequence(123)
            .with_decimals(2)
            .with_unit(SIPrefix::KILO, SIUnit::HERTZ)
            .with_range(0.0, 100.0)
            .build()
            .expect("Failed to build NumberBuffer");

        assert!(buffer.is_builded());
        assert_eq!(buffer.value(), 42.5);
        assert_eq!(buffer.sequence(), 123);
        assert_eq!(buffer.decimals(), 2);
        assert_eq!(buffer.unit(), Some((SIPrefix::KILO, SIUnit::HERTZ)));
        assert_eq!(buffer.range(), Some((0.0, 100.0)));
    }

    #[test]
    fn test_number_buffer_from_f64() {
        let buffer: NumberBuffer = 3.14.into();
        let built_buffer = buffer
            .with_source(1)
            .with_sequence(456)
            .build()
            .expect("Failed to build NumberBuffer");

        assert_eq!(built_buffer.value(), 3.14);
    }

    #[test]
    fn test_number_buffer_to_f64() {
        let buffer = NumberBuffer::new()
            .with_value(98.6)
            .with_source(1)
            .with_sequence(789)
            .build()
            .expect("Failed to build NumberBuffer");

        let value: f64 = buffer.into();
        assert_eq!(value, 98.6);
    }

    #[test]
    fn test_number_buffer_with_whitelist() {
        let whitelist = vec![1.0, 2.0, 3.0, 4.0];
        let buffer = NumberBuffer::new()
            .with_value(2.0)
            .with_source(1)
            .with_sequence(999)
            .with_whitelist(whitelist.clone())
            .build()
            .expect("Failed to build NumberBuffer");

        assert_eq!(buffer.whitelist(), Some(whitelist));
    }
}
