use futures::{
    future::{BoxFuture, FutureExt}, 
    task::{waker_ref, ArcWake}, 
}; 

use std::{
    future::Future, 
    sync::mpsc::{sync_channel, Receiver, SyncSender}, 
    sync::{Arc, Mutex}, 
    task::Context, 
    time::Duration, 
}; 

use timer_future::TimerFuture; 

struct Executor {
    ready_queue: Receiver<Arc<Task>>, 
}

#[derive(Clone)]
struct Spawner {
    task_sender: SyncSender<Arc<Task>>, 
}

struct Task {
    future: Mutex<Option<BoxFuture<'static, ()>>>,
    task_sender: SyncSender<Arc<Task>>, 
}

fn new_executor_and_spawner() -> (Executor, Spawner) {
    const MAX_QUEUED_TASKS: usize = 10_000; 
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
    (Executor {ready_queue}, Spawner {task_sender})
}

impl Spawner {
    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed(); 
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)), 
            task_sender: self.task_sender.clone(), 
        }); 
        self.task_sender.send(task).expect("too many tasks queued"); 
    }
}

// ArcWake is a trait provided by the futures::task module, and it is used in combination with Arc to implement task waking for async tasks.
impl ArcWake for Task {
    // Implement the `wake_by_ref` method required by the `ArcWake` trait.
    fn wake_by_ref(arc_self: &Arc<Self>) {
        // Clone the `Arc` reference, creating a new strong reference.
        let cloned = arc_self.clone();

        // Access the `task_sender` field of the `Task` struct.
        // This sender is used to send the task to a queue for execution.
        arc_self
            .task_sender
            .send(cloned)
            .expect("too many tasks queued");
    }
}


impl Executor {
    fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() { 
                let waker = waker_ref(&task); 
                let context = &mut Context::from_waker(&waker);
                if future.as_mut().poll(context).is_pending() {
                    *future_slot = Some(future); 
                }
            }
        }
    }
}

fn main() {
    let (executor, spawner) = new_executor_and_spawner(); 

    spawner.spawn(async { 
        println!("spawn task"); 
        TimerFuture::new(Duration::from_secs(1)).await;
        println!("done"); 
    }); 

    drop(spawner); 

    executor.run(); 
}