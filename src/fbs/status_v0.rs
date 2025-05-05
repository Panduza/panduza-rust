#[allow(warnings)]
pub mod status_v0_generated;

use bytes::Bytes;
use status_v0_generated::{Status, StatusArgs, Timestamp};
use std::time::{SystemTime, UNIX_EPOCH};

pub enum InstanceState {}

pub struct InstanceStatus {
    ///
    ///
    pub state: InstanceState,
}

#[derive(Debug)]
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
    pub fn from_args(r#type: StatusType, source: String, message: String) -> Self {
        let mut builder = flatbuffers::FlatBufferBuilder::new();

        let source = builder.create_string(&source);
        let message = builder.create_string(&message);

        let timestamp = Self::generate_timestamp();

        let object = Status::create(
            &mut builder,
            &StatusArgs {
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
    pub fn object(&self) -> Status {
        flatbuffers::root::<Status>(&self.raw_data).unwrap()
    }
}
