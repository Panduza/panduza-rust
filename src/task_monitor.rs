use std::sync::{Arc, Mutex};
use std::time::Duration;

use tokio::sync::mpsc::{channel, Receiver, Sender};

use tokio::task::JoinHandle;

///
///
pub enum Event {}

pub type TaskHandle = JoinHandle<Result<(), String>>;

///
///
pub struct TaskMonitor {
    handles: Arc<Mutex<Vec<TaskHandle>>>,

    handle_sender: Sender<TaskHandle>,

    task_feeding: JoinHandle<()>,
    task_monitoring: JoinHandle<()>,
}

impl TaskMonitor {
    pub fn new() -> (Self, Receiver<Event>) {
        let handles = Arc::new(Mutex::new(Vec::new()));

        let (event_sender, event_receiver) = channel::<Event>(10);

        let (handle_sender, mut handle_receiver) = channel::<TaskHandle>(10);

        //
        //
        let handles_clone_1 = handles.clone();
        let feed = tokio::spawn(async move {
            loop {
                match handle_receiver.recv().await {
                    Some(new_handle) => {
                        println!("new {:?}", new_handle.id());
                        handles_clone_1.lock().unwrap().push(new_handle);
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
                let mut hlock = handles_clone_2.lock().unwrap();
                for h in hlock.iter_mut() {
                    println!("{:?}", h.is_finished());
                    let r = h.await;
                    // h.abort();
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
