pub mod boolean;
pub mod bytes;
pub mod data_pack;
pub mod json;
pub mod notification;
pub mod number;
pub mod si;
pub mod status;
pub mod string;


/// Unique identifier for callbacks
pub type CallbackId = u64;

/// Type alias for callback function with generic type T
pub type CallbackFn<T> = Box<dyn Fn(&T) + Send + Sync>;

/// Type alias for condition function that filters events with generic type T
pub type ConditionFn<T> = Box<dyn Fn(&T) -> bool + Send + Sync>;

/// Callback entry containing the callback and optional condition
pub struct CallbackEntry<T> {
    pub callback: CallbackFn<T>,
    pub condition: Option<ConditionFn<T>>,
}

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

