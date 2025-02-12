use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

/// Payload of event enums
///
pub struct EventBody {
    /// Name of the task
    ///
    pub task_name: String,

    /// Id of the task
    ///
    pub task_id: String,

    /// Error message if any
    ///
    pub error_message: Option<String>,
}

///
///
pub enum Event {
    TaskMonitorError(String),
    TaskCreated(EventBody),
    TaskStopProperly(EventBody),
    TaskStopWithPain(EventBody),
    TaskPanicOMG(EventBody),
}

/// Type of handle managed by this monitor
///
pub type TaskHandle = JoinHandle<Result<(), String>>;

/// Message to request a task monitoring
///
/// First parameter of the tupple is the name of the task
/// The second is the handle of the task
///
pub type NamedTaskHandle = (String, TaskHandle);

/// This object is able to monitor a group of tokio task and report status
///
pub struct TaskMonitor {
    /// List of the monitored task handles
    ///
    handles: Arc<Mutex<Vec<NamedTaskHandle>>>,

    /// Sender that allow other task to send their handle for monitoring
    ///
    handle_sender: Sender<NamedTaskHandle>,

    /// Internal handle for the task that feed this monitor
    ///
    task_feeding: JoinHandle<()>,

    /// Internal handle for the task that monitor other tasks
    ///
    task_monitoring: JoinHandle<()>,
}

// ----------------------------------------------------------------------------
impl TaskMonitor {
    pub fn new() -> (Self, Receiver<Event>) {
        //
        // Initialize handles
        let handles = Arc::new(Mutex::new(Vec::new()));

        //
        // Initialize events channel to alert the parent listener
        let (event_sender, event_receiver) = channel::<Event>(10);

        //
        // Initialize handles channel to allow other object to send task handles to monitor
        let (handle_sender, mut handle_receiver) = channel::<NamedTaskHandle>(10);

        //
        // TASK to register new handles
        let handles_clone_1 = handles.clone();
        let event_sender_1 = event_sender.clone();
        let feed_t = tokio::spawn(async move {
            loop {
                match handle_receiver.recv().await {
                    Some(new_handle) => {
                        // Send an event
                        event_sender_1
                            .send(Event::TaskCreated(EventBody {
                                task_name: new_handle.0.clone(),
                                task_id: new_handle.1.id().to_string(),
                                error_message: None,
                            }))
                            .await
                            .unwrap();

                        // Save the handle
                        handles_clone_1.lock().await.push(new_handle);
                    }
                    None => todo!(),
                }
            }
        });

        //
        // TASK to monitor other tasks
        let handles_clone_2 = handles.clone();
        let monitor_t = tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_millis(2000)).await;
                let mut hlock = handles_clone_2.lock().await;
                let mut i = 0;

                if hlock.len() <= 0 {
                    println!("no more task to monitor")
                }

                while i < hlock.len() {
                    let h = &mut hlock[i];
                    println!("{:?}", h.1.is_finished());
                    if h.1.is_finished() {
                        let element = hlock.remove(i);

                        let r = element.1.await;
                        println!("Task finished: {:?}", r);
                        // Supprimer l'élément du vecteur
                    } else {
                        // Incrémenter l'index seulement si l'élément n'a pas été supprimé
                        i += 1;
                    }
                }
                drop(hlock);
            }
        });

        (
            Self {
                handles: handles,
                handle_sender: handle_sender,
                task_feeding: feed_t,
                task_monitoring: monitor_t,
            },
            event_receiver,
        )
    }

    ///
    ///
    pub async fn cancel_all_monitored_tasks(&mut self) {
        let mut hlock = self.handles.lock().await;

        for h in hlock.iter_mut() {
            h.1.abort();
        }

        let mut i = 0;
        while i < hlock.len() {
            let element = hlock.remove(i);
            element.1.await.unwrap().unwrap();
            i += 1;
        }

        hlock.clear();
    }

    ///
    ///
    pub fn handle_sender(&self) -> Sender<NamedTaskHandle> {
        self.handle_sender.clone()
    }
}
