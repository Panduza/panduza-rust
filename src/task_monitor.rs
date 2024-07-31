use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

/// Delay between 2 monitoring action
///
static DELAY_MS_BETWEEN_MONITORING: u64 = 200;

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

/// Monitoring events
///
pub enum Event {
    TaskMonitorError(String),
    TaskCreated(EventBody),
    TaskStopProperly(EventBody),
    TaskStopWithPain(EventBody),
    TaskPanicOMG(EventBody),
    NoMoreTask,
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

/// Object to send new task handle to the monitor
///
pub type TaskMonitorLink = Sender<NamedTaskHandle>;

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
    /// Create a new task monitor
    ///
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
                        if let Err(e) = event_sender_1
                            .send(Event::TaskCreated(EventBody {
                                task_name: new_handle.0.clone(),
                                task_id: new_handle.1.id().to_string(),
                                error_message: None,
                            }))
                            .await
                        {
                            println!("{:?} - TaskMonitor warning ! {:?}", line!(), e.to_string());
                        }

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
        let event_sender_2 = event_sender.clone();
        let monitor_t = tokio::spawn(async move {
            loop {
                // Wait before next monitoring
                tokio::time::sleep(Duration::from_millis(DELAY_MS_BETWEEN_MONITORING)).await;

                // Lock handles
                let mut hlock = handles_clone_2.lock().await;

                // No more task to check
                if hlock.len() <= 0 {
                    if let Err(e) = event_sender_2.send(Event::NoMoreTask).await {
                        println!("{:?} - TaskMonitor warning ! {:?}", line!(), e.to_string());
                    }
                }

                // Monitor tasks, checking for each task if it is finished
                let mut i = 0;
                while i < hlock.len() {
                    let h = &mut hlock[i];
                    if h.1.is_finished() {
                        let element = hlock.remove(i);
                        let task_id = element.1.id().to_string();
                        match element.1.await {
                            Ok(result) => match result {
                                //
                                // Task end properly
                                Ok(_) => {
                                    if let Err(e) = event_sender_2
                                        .send(Event::TaskStopProperly(EventBody {
                                            task_name: element.0,
                                            task_id: task_id,
                                            error_message: None,
                                        }))
                                        .await
                                    {
                                        println!(
                                            "{:?} - TaskMonitor warning ! {:?}",
                                            line!(),
                                            e.to_string()
                                        );
                                    }
                                }
                                //
                                // Task end with an error
                                Err(e) => {
                                    if let Err(e) = event_sender_2
                                        .send(Event::TaskStopWithPain(EventBody {
                                            task_name: element.0,
                                            task_id: task_id,
                                            error_message: Some(e.to_string()),
                                        }))
                                        .await
                                    {
                                        println!(
                                            "{:?} - TaskMonitor warning ! {:?}",
                                            line!(),
                                            e.to_string()
                                        );
                                    }
                                }
                            },
                            //
                            // Task PANIC
                            Err(e) => {
                                if let Err(e) = event_sender_2
                                    .send(Event::TaskPanicOMG(EventBody {
                                        task_name: element.0,
                                        task_id: task_id,
                                        error_message: Some(e.to_string()),
                                    }))
                                    .await
                                {
                                    println!(
                                        "{:?} - TaskMonitor warning ! {:?}",
                                        line!(),
                                        e.to_string()
                                    );
                                }
                            }
                        }
                    } else {
                        // Incrémenter l'index seulement si l'élément n'a pas été supprimé
                        i += 1;
                    }
                }

                // Release handles
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

    /// Cancel all tasks
    ///
    pub async fn cancel_all_monitored_tasks(&mut self) {
        // lock elements
        let mut hlock = self.handles.lock().await;

        // abort all tasks
        for h in hlock.iter_mut() {
            h.1.abort();
        }

        // Wait for them to stop
        // we do not care about the status
        let mut i = 0;
        while i < hlock.len() {
            let element = hlock.remove(i);
            let _ = element.1.await;
            i += 1;
        }

        // vector should be empty here but...
        hlock.clear();
    }

    /// Provides access to the handler sender to send new handle to monitor
    ///
    pub fn handle_sender(&self) -> Sender<NamedTaskHandle> {
        self.handle_sender.clone()
    }

    /// Provides access to the handler sender to send new handle to monitor
    ///
    pub async fn stop(self) {
        self.task_feeding.abort();
        self.task_monitoring.abort();
        self.task_feeding.await.unwrap();
        self.task_monitoring.await.unwrap();
    }
}
