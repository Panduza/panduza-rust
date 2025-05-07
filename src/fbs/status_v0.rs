#[allow(warnings)]
pub mod status_v0_generated;

use bytes::Bytes;
use status_v0_generated::{InstanceStatus, InstanceStatusArgs, Status, StatusArgs, Timestamp};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::InstanceState;

#[derive(Debug)]
pub struct InstanceStatusBuffer {
    /// Internal Raw Data
    ///
    raw_data: Bytes,
}

impl InstanceStatusBuffer {
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

    // ///
    // ///
    // fn generate_timestamp() -> Timestamp {
    //     let now = SystemTime::now();
    //     let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    //     let seconds = since_the_epoch.as_secs();
    //     let nanoseconds = since_the_epoch.subsec_nanos();
    //     Timestamp::new(seconds, nanoseconds)
    // }

    ///
    ///
    pub fn from_args(name: String, state: InstanceState, error_string: Option<String>) -> Self {
        let mut builder = flatbuffers::FlatBufferBuilder::new();

        let instance_name = builder.create_string(&name);
        let error_string = error_string.map(|e| builder.create_string(&e));

        // Create the instance object - actual implementation depends on the generated code
        let object = InstanceStatus::create(
            &mut builder,
            &InstanceStatusArgs {
                instance: Some(instance_name),
                state: state as u16, // Assuming state is an enum or similar
                error_string,
            },
        );

        builder.finish(object, None);

        let raw_data = Bytes::from(builder.finished_data().to_vec());

        // Here we copy into the buffer
        Self { raw_data }
    }

    ///
    ///
    pub fn object(&self) -> InstanceStatus {
        flatbuffers::root::<InstanceStatus>(&self.raw_data).unwrap()
    }
}

#[derive(Clone, Debug, Default)]
///
///
pub struct StatusBuffer {
    /// Internal Raw Data
    ///
    raw_data: Bytes,
}

impl StatusBuffer {
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
    pub fn from_args(instances: Vec<InstanceStatusBuffer>) -> Self {
        let mut builder = flatbuffers::FlatBufferBuilder::new();

        // Create a vector of instance offsets
        let mut instance_offsets = Vec::new();
        for instance_buffer in &instances {
            // Extract data from each instance buffer
            let instance_status = instance_buffer.object();

            // Copy data to new builder
            let instance_name = if let Some(name) = instance_status.instance() {
                Some(builder.create_string(name))
            } else {
                None
            };

            let error_string = if let Some(err) = instance_status.error_string() {
                Some(builder.create_string(err))
            } else {
                None
            };

            let offset = InstanceStatus::create(
                &mut builder,
                &InstanceStatusArgs {
                    instance: instance_name,
                    state: instance_status.state(),
                    error_string,
                },
            );

            instance_offsets.push(offset);
        }

        // Create vector of offsets
        let instances_vector = builder.create_vector(&instance_offsets);

        let timestamp = Self::generate_timestamp();

        // Create the Status object with the vector of instances
        let object = Status::create(
            &mut builder,
            &StatusArgs {
                instances: Some(instances_vector),
                timestamp: Some(&timestamp),
            },
        );

        builder.finish(object, None);

        let raw_data = Bytes::from(builder.finished_data().to_vec());

        Self { raw_data }
    }

    ///
    ///
    pub fn object(&self) -> Status {
        flatbuffers::root::<Status>(&self.raw_data).unwrap()
    }

    ///
    ///
    pub fn all_instances_are_running(&self) -> Result<bool, &'static str> {
        // Get the last value
        let value = self.object();

        // Check if we have a value
        if let Some(instances) = value.instances() {
            if instances.is_empty() {
                return Err("No instances found");
            }

            // Check if all instances are running
            for instance in instances {
                if instance.state() != InstanceState::Running as u16 {
                    return Ok(false);
                }
            }
            return Ok(true);
        }

        // No instances found
        Err("No instances found")
    }

    pub fn at_least_one_instance_is_not_running(
        &self,
    ) -> Result<bool, &'static str> {
        // Get the last value
        let value = self.object();

        // Check if we have a value
        if let Some(instances) = value.instances() {
            if instances.is_empty() {
                return Err("No instances found");
            }

            // Check if at least one instance is not running
            for instance in instances {
                if instance.state() != InstanceState::Running as u16 {
                    return Ok(true);
                }
            }
            return Ok(false);
        }

        // No instances found
        Err("No instances found")
    }
}
