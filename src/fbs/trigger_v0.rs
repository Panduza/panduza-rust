pub mod trigger_v0_generated;

use bytes::Bytes;
use std::time::{SystemTime, UNIX_EPOCH};
use trigger_v0_generated::{Options, OptionsArgs, Range, Timestamp, Trigger, TriggerArgs};

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
    pub fn from_values(refresh: f32, option_id: u8, range: Option<(f32, f32)>, whitelist: Option<&Vec<f32>>) -> Self {

        let mut builder = flatbuffers::FlatBufferBuilder::new();

        let whitelist_flat = 
        if let Some(wl) = whitelist {
            Some(builder.create_vector(wl))
        } else {
            None
        };

        
        let range_flat = 
        if let Some(r) = range {
            Some(Range::new(true, r.0, r.1))
        }
        else {
            None
        };
        
        let options_flat = Options::create(&mut builder, &OptionsArgs{
            id: option_id,
            range: range_flat.as_ref(),
            whitelist: whitelist_flat
        });

        let timestamp = Self::generate_timestamp();

        let object = Trigger::create(
            &mut builder,
            &TriggerArgs {
                refresh: refresh,
                timestamp: Some(&timestamp),
                options: Some(options_flat),
            },
        );

        builder.finish(object, None);

        let raw_data = Bytes::from(builder.finished_data().to_vec());

        // Here we copy into the buffer
        Self { raw_data: raw_data }
    }
}
