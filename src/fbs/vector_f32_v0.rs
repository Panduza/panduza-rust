#[allow(warnings)]
pub mod vector_f32_v0_generated;

use bytes::Bytes;
use std::time::{SystemTime, UNIX_EPOCH};
use vector_f32_v0_generated::{Timestamp, VectorF32, VectorF32Args};

#[derive(Debug)]
///
///
pub struct VectorF32Buffer {
    /// Internal Raw Data
    ///
    raw_data: Bytes,
}

impl VectorF32Buffer {
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
    pub fn from_values(values: &Vec<f32>) -> Self {
        let mut builder = flatbuffers::FlatBufferBuilder::new();

        let values = builder.create_vector(values);

        let timestamp = Self::generate_timestamp();

        let object = VectorF32::create(
            &mut builder,
            &VectorF32Args {
                values: Some(values),
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
    pub fn object(&self) -> VectorF32 {
        flatbuffers::root::<VectorF32>(&self.raw_data).unwrap()
    }
}
