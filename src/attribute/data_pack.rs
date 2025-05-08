use std::sync::Arc;
use tokio::sync::Notify;

/// Object that represent the data pack for the attribute
/// 
/// This object stores incoming data in its internal data queue.
/// The queue has a limited capacity, and when it is full, the oldest data is removed to make room for new data.
/// 
#[derive(Debug, Default)]
pub struct AttributeDataPack<T: Clone> {
    /// Queue of value
    ///
    queue: Vec<T>,

    /// Last value received
    /// 
    last: Option<T>,

    /// Update notifier
    ///
    update_notifier: Arc<Notify>,
}

impl<T: Clone> AttributeDataPack<T> {
    ///
    ///
    pub fn push(&mut self, v: T) {
        self.queue.push(v);
        self.update_notifier.notify_waiters();
    }

    ///
    ///
    pub fn last(&mut self) -> Option<T> {
        while self.pop().is_some() {
            // pop all values
        }
        self.last.clone()
    }

    ///
    ///
    pub fn pop(&mut self) -> Option<T> {
        if self.queue.is_empty() {
            None
        } else {
            self.last = Some(self.queue.remove(0));
            self.last.clone()
        }
    }

    ///
    ///
    pub fn update_notifier(&self) -> Arc<Notify> {
        self.update_notifier.clone()
    }
}
