use thiserror::Error;

pub mod boolean;
pub mod bytes;
pub mod data_pack;
pub mod notification;
pub mod number;
pub mod status;
pub mod string;
pub mod structure;

/// Standard message attribute for sending messages
/// The attribute manages a value than change over time
pub mod std_obj;

/// The attribute manages a RO stream of data
///
pub mod ro_stream;

/// Error type for attribute operations
#[derive(Error, Debug)]
pub enum AttributeError {
    #[error("Element not found: {0}")]
    NotFound(String),
    #[error("Invalid type: expect:{0} found:{1}")]
    InvalidType(String, String),
}

/// Unique identifier for callbacks
pub type CallbackId = u64;

/// Type alias for asynchronous callback function with generic type T
pub type CallbackFn<T> =
    Box<dyn Fn(T) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> + Send + Sync>;

/// Type alias for condition function that filters events with generic type T
pub type ConditionFn<T> = Box<dyn Fn(&T) -> bool + Send + Sync>;

/// Asynchronous callback entry containing the callback and optional condition
pub struct CallbackEntry<T> {
    pub callback: CallbackFn<T>,
    pub condition: Option<ConditionFn<T>>,
}

impl<T> std::fmt::Debug for CallbackEntry<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CallbackEntry")
            .field("callback", &"<async callback function>")
            .field(
                "condition",
                &if self.condition.is_some() {
                    "<condition function>"
                } else {
                    "None"
                },
            )
            .finish()
    }
}
