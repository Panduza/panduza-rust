use std::fmt::Display;

use crate::MessagePayloadManager;

#[derive(Copy, Clone, PartialEq)]
pub struct BooleanMessage {
    value: bool,
}

impl Into<BooleanMessage> for bool {
    fn into(self) -> BooleanMessage {
        return BooleanMessage { value: true };
    }
}

impl From<Vec<u8>> for BooleanMessage {
    fn from(value: Vec<u8>) -> Self {
        return BooleanMessage { value: true };
    }
}
impl Into<Vec<u8>> for BooleanMessage {
    fn into(self) -> Vec<u8> {
        return vec![1];
    }
}

impl Display for BooleanMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.value))
    }
}

impl MessagePayloadManager for BooleanMessage {}
