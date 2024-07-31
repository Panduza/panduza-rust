pub mod sample_generated;
use bytes::Bytes;
use sample_generated::{Sample, SampleArgs};

#[derive(Debug)]
///
///
pub struct SampleBuffer {
    /// Internal Raw Data
    ///
    raw_data: Bytes,
}

impl SampleBuffer {
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
    pub fn from_values(values: &Vec<f32>) -> Self {
        let mut builder = flatbuffers::FlatBufferBuilder::new();

        // https://github.com/google/flatbuffers/blob/master/samples/sample_binary.rs

        let inventory = builder.create_vector(values);

        let orc = Sample::create(
            &mut builder,
            &SampleArgs {
                values: Some(inventory),
            },
        );

        builder.finish(orc, None);

        let raw_data = Bytes::from(builder.finished_data().to_vec());

        // Here we copy into the buffer
        Self { raw_data: raw_data }
    }
}
