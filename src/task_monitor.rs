use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

///
///
pub enum Event {}

/// Type of handle managed by this monitor
///
pub type TaskHandle = JoinHandle<Result<(), String>>;

/// This object is able to monitor a group of tokio task and report status
///
pub struct TaskMonitor {
    /// List of the monitored task handles
    ///
    handles: Arc<Mutex<Vec<TaskHandle>>>,

    /// Sender that allow other task to send their handle for monitoring
    ///
    handle_sender: Sender<TaskHandle>,

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
        let (handle_sender, mut handle_receiver) = channel::<TaskHandle>(10);

        //
        // TASK
        let handles_clone_1 = handles.clone();
        let feed = tokio::spawn(async move {
            loop {
                match handle_receiver.recv().await {
                    Some(new_handle) => {
                        println!("new {:?}", new_handle.id());
                        handles_clone_1.lock().await.push(new_handle);
                    }
                    None => todo!(),
                }
            }
        });

        //
        //
        let handles_clone_2 = handles.clone();
        let to = tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_millis(2000)).await;
                let mut hlock = handles_clone_2.lock().await;
                let mut i = 0;

                if hlock.len() <= 0 {
                    println!("no more task to monitor")
                }

                while i < hlock.len() {
                    let h = &mut hlock[i];
                    println!("{:?}", h.is_finished());
                    if h.is_finished() {
                        let r = h.await;
                        println!("Task finished: {:?}", r);
                        // Supprimer l'élément du vecteur
                        hlock.remove(i);
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
                task_feeding: feed,
                task_monitoring: to,
            },
            event_receiver,
        )
    }

    ///
    ///
    pub async fn cancel_all_monitored_tasks(&mut self) {
        let mut hlock = self.handles.lock().await;

        for h in hlock.iter_mut() {
            h.abort();
        }

        for h in hlock.iter_mut() {
            h.await.unwrap().unwrap();
        }

        hlock.clear();
    }

    ///
    ///
    pub fn handle_sender(&self) -> Sender<TaskHandle> {
        self.handle_sender.clone()
    }
}

// Task 1
// loop
//      sleep
//      check handles
//              if one is dead => send a message
//
// => share the status receiver

// Task 2
// loop
//      wait for incoming handle
//          register handle
//
// => share the handle sender
