pub mod async_generic;
pub mod boolean;
pub mod bytes;
pub mod data_pack;
pub mod generic;
pub mod json;
pub mod notification;
pub mod number;
pub mod si;
pub mod status;
pub mod string;

/// Unique identifier for callbacks
pub type CallbackId = u64;

/// Type alias for synchronous callback function with generic type T
pub type CallbackFn<T> = Box<dyn Fn(&T) + Send + Sync>;

/// Type alias for asynchronous callback function with generic type T
pub type AsyncCallbackFn<T> =
    Box<dyn Fn(T) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> + Send + Sync>;

/// Type alias for condition function that filters events with generic type T
pub type ConditionFn<T> = Box<dyn Fn(&T) -> bool + Send + Sync>;

/// Callback entry containing the callback and optional condition
pub struct CallbackEntry<T> {
    pub callback: CallbackFn<T>,
    pub condition: Option<ConditionFn<T>>,
}

/// Asynchronous callback entry containing the callback and optional condition
pub struct AsyncCallbackEntry<T> {
    pub callback: AsyncCallbackFn<T>,
    pub condition: Option<ConditionFn<T>>,
}

// Export the async generic attribute for easier access
pub use async_generic::AsyncGenericAttribute;

impl<T> std::fmt::Debug for CallbackEntry<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CallbackEntry")
            .field("callback", &"<callback function>")
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

impl<T> std::fmt::Debug for AsyncCallbackEntry<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AsyncCallbackEntry")
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
