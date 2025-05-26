use super::panduza_generated::panduza::Timestamp;
use std::time::{SystemTime, UNIX_EPOCH};

/// Generates a timestamp for message headers using the current system time
///
/// # Returns
/// A Timestamp object with current time in seconds and nanoseconds
pub fn generate_timestamp() -> Timestamp {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let seconds = since_the_epoch.as_secs();
    let nanoseconds = since_the_epoch.subsec_nanos();
    Timestamp::new(seconds, nanoseconds)
}
