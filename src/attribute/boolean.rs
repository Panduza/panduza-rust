use crate::fbs::BooleanBuffer;
use crate::pubsub::Publisher;
use crate::reactor::DataReceiver;
use crate::AttributeMode;
use bytes::Bytes;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use tokio::sync::Notify;
use tokio::time::timeout;

use super::data_pack::AttributeDataPack;

/// Type alias for callback function
pub type CallbackFn = Box<dyn Fn(bool) + Send + Sync>;

/// Type alias for condition function that filters events
pub type ConditionFn = Box<dyn Fn(bool) -> bool + Send + Sync>;

/// Callback entry containing the callback and optional condition
pub struct CallbackEntry {
    pub callback: CallbackFn,
    pub condition: Option<ConditionFn>,
}

impl std::fmt::Debug for CallbackEntry {
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

/// Unique identifier for callbacks
pub type CallbackId = u64;



// TODO: la logique de gestion va changer
// au lieu d'avoir un data pack, on va passer par un systéme de callback
// Les attributs sont assez simimilaires, on peut donc les gérer de manière générique
// L'élément changeant est souvent juste le type de données
// Il faut manipuler des données de types "Bytes" ou "Zbytes"
// Une fonction permettra d'assurer une initialisation correcte



#[derive(Clone, Debug)]
/// Object to manage the BooleanAttribute
///
pub struct BooleanAttribute {
    ///
    /// TODO: maybe add this into the data pack
    topic: String,

    mode: AttributeMode,

    /// Object that all the attribute to publish
    ///
    cmd_publisher: Publisher,

    /// Initial data
    ///
    pack: Arc<Mutex<AttributeDataPack<BooleanBuffer>>>,
    /// Update notifier
    ///
    update_notifier: Arc<Notify>,

    /// Callbacks storage
    ///
    callbacks: Arc<Mutex<HashMap<CallbackId, CallbackEntry>>>,

    /// Next callback ID
    ///
    next_callback_id: Arc<Mutex<CallbackId>>,
    //
    // on_message (trigger when a new message is received)
}

impl BooleanAttribute {
    /// Create a new instance
    ///
    pub async fn new(
        topic: String,
        mode: AttributeMode,
        cmd_publisher: Publisher,
        mut att_receiver: DataReceiver,
    ) -> Self {
        //
        // Create data pack
        let pack = Arc::new(Mutex::new(AttributeDataPack::<BooleanBuffer>::default())); //
                                                                                        //
        let update_1 = pack.lock().unwrap().update_notifier();
        // Initialize callbacks storage
        let callbacks: Arc<Mutex<HashMap<CallbackId, CallbackEntry>>> =
            Arc::new(Mutex::new(HashMap::new())); // Wait for the first message if mode is not readonly
        if mode != AttributeMode::WriteOnly {
            //
            // Create the recv task
            let pack_2 = pack.clone();
            let callbacks_for_task = callbacks.clone();

            tokio::spawn({
                let topic = topic.clone();
                async move {
                    loop {
                        //
                        let message = att_receiver.recv().await;

                        println!("new message on topic {:?}: {:?}", &topic, message);

                        // Manage message
                        if let Some(message) = message {
                            // Push into pack
                            let boolean_buffer = BooleanBuffer::from_raw_data(message);
                            let value = bool::from(boolean_buffer.clone());
                            pack_2.lock().unwrap().push(boolean_buffer);

                            // Trigger callbacks
                            let callbacks_map = callbacks_for_task.lock().unwrap();
                            for (_id, callback_entry) in callbacks_map.iter() {
                                // Check condition if present
                                let should_trigger =
                                    if let Some(ref condition) = callback_entry.condition {
                                        condition(value)
                                    } else {
                                        true
                                    };

                                if should_trigger {
                                    (callback_entry.callback)(value);
                                }
                            }
                        }
                        // None => no more message
                        else {
                            break;
                        }
                    }
                }
            });

            // Need a timeout here
            update_1.notified().await;
        } //
          // Return attribute
        Self {
            topic: topic,
            cmd_publisher: cmd_publisher,
            pack: pack,
            update_notifier: update_1,
            mode: mode,
            callbacks: callbacks,
            next_callback_id: Arc::new(Mutex::new(0)),
        }
    }

    /// Send command and do not wait for validation
    ///
    pub async fn shoot(&mut self, value: bool) {
        // Wrap value into payload
        let pyl = Bytes::from(serde_json::to_string(&value).unwrap());

        // Send the command
        self.cmd_publisher.publish(pyl).await.unwrap();
    }

    /// Notify when new data have been received
    ///
    pub fn update_notifier(&self) -> Arc<Notify> {
        self.update_notifier.clone()
    }

    ///
    ///
    pub async fn set(&mut self, value: bool) -> Result<(), String> {
        //
        self.shoot(value).await;

        if self.mode == AttributeMode::ReadWrite {
            let delay = Duration::from_secs(5);

            // Wait for change in the data pack
            timeout(delay, self.update_notifier.notified())
                .await
                .map_err(|e| e.to_string())?;

            while value != <bool>::from(self.get().unwrap()) {
                // append 3 retry before failling if update received but not good
                timeout(delay, self.update_notifier.notified())
                    .await
                    .map_err(|e| e.to_string())?;
            }
        }

        Ok(())
    }

    ///
    ///
    pub fn get(&self) -> Option<BooleanBuffer> {
        self.pack.lock().unwrap().last()
    }

    ///
    ///
    pub fn pop(&self) -> Option<BooleanBuffer> {
        self.pack.lock().unwrap().pop()
    }

    pub async fn wait_for_value(&self, value: bool) -> Result<(), String> {
        if self.mode == AttributeMode::WriteOnly {
            return Err("Cannot wait for value in WriteOnly mode".to_string());
        }

        while let Some(last_value) = self.get() {
            if bool::from(last_value) == value {
                return Ok(());
            }
            self.update_notifier.notified().await;
        }
        Ok(())
    }
    /// Add a callback that will be triggered when a new message is received
    /// Returns a callback ID that can be used to remove the callback later
    ///
    pub fn add_callback<F>(&self, callback: F) -> CallbackId
    where
        F: Fn(bool) + Send + Sync + 'static,
    {
        let no_condition: Option<fn(bool) -> bool> = None;
        self.add_callback_with_condition(callback, no_condition)
    }

    /// Add a callback with a condition filter
    /// The callback will only be triggered if the condition returns true
    /// Returns a callback ID that can be used to remove the callback later
    ///
    pub fn add_callback_with_condition<F, C>(&self, callback: F, condition: Option<C>) -> CallbackId
    where
        F: Fn(bool) + Send + Sync + 'static,
        C: Fn(bool) -> bool + Send + Sync + 'static,
    {
        let mut callbacks_map = self.callbacks.lock().unwrap();
        let mut next_id = self.next_callback_id.lock().unwrap();

        let callback_id = *next_id;
        *next_id += 1;

        let callback_entry = CallbackEntry {
            callback: Box::new(callback),
            condition: condition.map(|c| Box::new(c) as ConditionFn),
        };

        callbacks_map.insert(callback_id, callback_entry);
        callback_id
    }

    /// Remove a callback by its ID
    /// Returns true if the callback was found and removed, false otherwise
    ///
    pub fn remove_callback(&self, callback_id: CallbackId) -> bool {
        let mut callbacks_map = self.callbacks.lock().unwrap();
        callbacks_map.remove(&callback_id).is_some()
    }

    /// Clear all callbacks
    ///
    pub fn clear_callbacks(&self) {
        let mut callbacks_map = self.callbacks.lock().unwrap();
        callbacks_map.clear();
    }

    /// Get the number of registered callbacks
    ///
    pub fn callback_count(&self) -> usize {
        let callbacks_map = self.callbacks.lock().unwrap();
        callbacks_map.len()
    }
}
