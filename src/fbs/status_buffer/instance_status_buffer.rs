use crate::{fbs::panduza_generated::panduza::InstanceStatusArgs, InstanceState};
use flatbuffers::FlatBufferBuilder;

#[derive(Default, Clone, Debug, PartialEq)]
/// InstanceStatusBuffer is a wrapper around a flatbuffer serialized Message with an InstanceStatus payload.
/// It provides methods to create, access, and manipulate instance status data.
pub struct InstanceStatusBuffer {
    /// Source of the status, typically a unique identifier for the instance.
    ///
    instance: Option<String>,

    /// State of the instance, represented as a u16.
    ///
    state: Option<u16>,

    /// Optional error string providing additional context about the instance status.
    ///
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

    // ------------------------------------------------------------------------

    pub fn set_instance(&mut self, instance: String) {
        self.instance = Some(instance);
    }

    // ------------------------------------------------------------------------

    /// Method to get the instance name
    ///
    pub fn instance(&self) -> Option<&String> {
        self.instance.as_ref()
    }

    // ------------------------------------------------------------------------

    /// Method to set the instance state
    ///
    /// # Arguments
    ///
    /// * `state` - A InstanceState representing the new state of the instance.
    ///
    pub fn set_state(&mut self, state: InstanceState) {
        self.state = Some(state as u16);
    }

    // ------------------------------------------------------------------------

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
