use crate::clients::Clients;
use crate::tasks::{run_task, Task};
use std::sync::Arc;
use tokio::runtime::Builder;
use tokio::sync::mpsc;
use tokio::task::LocalSet;

#[derive(Clone)]
pub struct LocalSpawner {
    send: mpsc::UnboundedSender<Task>,
}

impl LocalSpawner {
    pub fn new() -> Self {
        let (send, mut recv) = mpsc::unbounded_channel();

        std::thread::spawn(move || {
            let rt = Builder::new_current_thread().enable_all().build().unwrap();
            let local = LocalSet::new();
            let shared_state = Arc::new(Clients::new());

            local.spawn_local(async move {
                while let Some(new_task) = recv.recv().await {
                    let state = shared_state.clone();
                    tokio::task::spawn_local(async move {
                        if let Err(e) = run_task(new_task, state).await {
                            eprintln!("Error running task: {:?}", e);
                        }
                    });
                }
            });

            rt.block_on(local);
        });

        Self { send }
    }

    pub fn spawn(&self, task: Task) {
        self.send
            .send(task)
            .expect("Thread with LocalSet has shut down.");
    }
}
