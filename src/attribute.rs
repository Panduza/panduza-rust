pub mod boolean;
pub mod bytes;
pub mod data_pack;
pub mod generic;
pub mod json;
pub mod notification;
pub mod number;
pub mod status;
pub mod string;

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

// Export the async generic attribute for easier access
pub use generic::GenericAttribute;

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
