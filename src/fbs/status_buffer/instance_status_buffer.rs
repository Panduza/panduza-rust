use crate::fbs::panduza_generated::panduza::InstanceStatusArgs;
use flatbuffers::{FlatBufferBuilder, WIPOffset};

#[derive(Default, Clone, Debug, PartialEq)]
/// InstanceStatusBuffer is a wrapper around a flatbuffer serialized Message with an InstanceStatus payload.
/// It provides methods to create, access, and manipulate instance status data.
pub struct InstanceStatusBuffer {
    instance: Option<String>,
    state: Option<u16>,
    error_string: Option<String>,
}

impl InstanceStatusBuffer {
    pub fn new() -> Self {
        Self {
            instance: None,
            state: None,
            error_string: None,
            // source: None,
            // sequence: None,
            // raw_data: None,
        }
    }

    pub fn from_args(instance: String, state: u16, error_string: Option<String>) -> Self {
        Self {
            instance: Some(instance),
            state: Some(state),
            error_string,
        }
    }

    pub fn to_fbs_args<'a>(&self, builder: &mut FlatBufferBuilder<'a>) -> InstanceStatusArgs<'a> {
        InstanceStatusArgs {
            instance: self.instance.as_ref().map(|s| builder.create_string(s)),
            state: self.state.unwrap_or(0),
            error_string: self.error_string.as_ref().map(|s| builder.create_string(s)),
        }
    }
}
