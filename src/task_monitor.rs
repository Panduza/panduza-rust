use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::channel;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

/// Delay between 2 monitoring action (in milliseconds)
///
const DELAY_MS_BETWEEN_MONITORING: u64 = 200;

/// Default capacity for event and handle channels
const DEFAULT_CHANNEL_CAPACITY: usize = 512;

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
/// Monitoring events
///
pub enum Event {
    /// Error related to the task monitor itself
    ///
    TaskMonitorError(String),

    /// Task created
    ///
    TaskCreated(EventBody),

    /// Task finished properly
    ///
    TaskStopProperly(EventBody),

    /// Task finished with an error
    ///
    TaskStopWithPain(EventBody),

    /// Task PANIC !
    ///
    TaskPanicOMG(EventBody),

    /// No more task to monitor
    ///
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

#[derive(Clone)]
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
    task_feeding: Arc<JoinHandle<()>>,

    /// Internal handle for the task that monitor other tasks
    ///
    task_monitoring: Arc<JoinHandle<()>>,
}

// ----------------------------------------------------------------------------
impl TaskMonitor {
    /// Create a new task monitor
    ///
    pub fn new<N: Into<String>>(name: N) -> (Self, Receiver<Event>) {
        //
        // Initialize the name of the task monitor
        let name = name.into();

        //
        // Initialize handles
        let handles = Arc::new(Mutex::new(Vec::new()));

        //
        // Initialize events channel to alert the parent listener
        let (event_sender, event_receiver) = channel::<Event>(DEFAULT_CHANNEL_CAPACITY);

        //
        // Initialize handles channel to allow other object to send task handles to monitor
        let (handle_sender, mut handle_receiver) =
            channel::<NamedTaskHandle>(DEFAULT_CHANNEL_CAPACITY);

        //
        // TASK to register new handles
        let feed_t = tokio::spawn({
            // Local clone variables
            let handles = handles.clone();
            let event_sender = event_sender.clone();
            let name = name.clone();

            // Task body
            async move {
                loop {
                    match handle_receiver.recv().await {
                        Some(new_handle) => {
                            // Send an event
                            if let Err(e) = event_sender
                                .send(Event::TaskCreated(EventBody {
                                    task_name: new_handle.0.clone(),
                                    task_id: new_handle.1.id().to_string(),
                                    error_message: None,
                                }))
                                .await
                            {
                                print_warning(&name, line!(), e);
                            }

                            // Save the handle
                            handles.lock().await.push(new_handle);
                        }
                        None => {
                            // Properly handle the closure of the channel
                            if let Err(e) = event_sender
                                .send(Event::TaskMonitorError(format!("Channel has been closed")))
                                .await
                            {
                                print_warning(&name, line!(), e);
                            }
                            break; // Exit the loop when the channel is closed
                        }
                    }
                }
            }
        });

        //
        // TASK to monitor other tasks
        let monitor_t = tokio::spawn({
            // Local clone variables for the task
            let handles = handles.clone();
            let event_sender = event_sender.clone();
            let name = name.clone();

            async move {
                // Track if we have already sent the NoMoreTask event
                let mut no_more_task_sent = false;

                loop {
                    // Wait before next monitoring
                    tokio::time::sleep(Duration::from_millis(DELAY_MS_BETWEEN_MONITORING)).await;

                    // Lock handles
                    let mut hlock = handles.lock().await;

                    // No more task to check
                    if hlock.is_empty() {
                        if !no_more_task_sent {
                            if let Err(e) = event_sender.send(Event::NoMoreTask).await {
                                print_warning(&name, line!(), e);
                            }
                            no_more_task_sent = true;
                        }
                    } else {
                        no_more_task_sent = false;
                    }

                    // Monitor tasks, checking for each task if it is finished
                    let mut i = 0;
                    while i < hlock.len() {
                        let h = &mut hlock[i];
                        if h.1.is_finished() {
                            let element = hlock.remove(i);
                            process_finished_task(&event_sender, &name, element).await;
                        } else {
                            // Increment the index only if the element has not been removed
                            i += 1;
                        }
                    }

                    // Release handles
                    drop(hlock);
                }
            }
        });

        (
            Self {
                handles: handles,
                handle_sender: handle_sender,
                task_feeding: Arc::new(feed_t),
                task_monitoring: Arc::new(monitor_t),
            },
            event_receiver,
        )
    }

    /// Cancel all tasks
    ///
    pub async fn cancel_all_monitored_tasks(&mut self) {
        println!("Cancelling all monitored tasks");
        // lock elements
        let mut hlock = self.handles.lock().await;

        // abort all tasks first
        for h in hlock.iter_mut() {
            println!("Aborting task: {:?}", h.0);
            h.1.abort();
        }

        // Then wait for them to complete
        for element in hlock.drain(..) {
            let _ = element.1.await;
        }

        // hlock est maintenant vide grâce à drain
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
        // self.task_feeding.await;
        // self.task_monitoring.await;
    }

    /// Returns the number of tasks currently being monitored
    ///
    pub async fn task_count(&self) -> usize {
        self.handles.lock().await.len()
    }
}

/// Print a warning message for this task monitor
///
fn print_warning(name: &String, line: u32, e: tokio::sync::mpsc::error::SendError<Event>) {
    println!(
        "{:?} - {:?} - TaskMonitor warning ! {:?}",
        &name,
        line,
        e.to_string()
    );
}

/// Process a finished task
///
async fn process_finished_task(event_sender: &Sender<Event>, name: &str, element: NamedTaskHandle) {
    let task_id = element.1.id().to_string();
    match element.1.await {
        Ok(result) => match result {
            // Task end properly
            Ok(_) => {
                if let Err(e) = event_sender
                    .send(Event::TaskStopProperly(EventBody {
                        task_name: element.0,
                        task_id,
                        error_message: None,
                    }))
                    .await
                {
                    print_warning(&name.to_string(), line!(), e);
                }
            }
            // Task end with an error
            Err(e) => {
                if let Err(e) = event_sender
                    .send(Event::TaskStopWithPain(EventBody {
                        task_name: element.0,
                        task_id,
                        error_message: Some(e),
                    }))
                    .await
                {
                    print_warning(&name.to_string(), line!(), e);
                }
            }
        },
        // Task PANIC
        Err(e) => {
            if let Err(e) = event_sender
                .send(Event::TaskPanicOMG(EventBody {
                    task_name: element.0,
                    task_id,
                    error_message: Some(e.to_string()),
                }))
                .await
            {
                print_warning(&name.to_string(), line!(), e);
            }
        }
    }
}
