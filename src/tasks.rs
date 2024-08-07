use crate::clients::Clients;
use crate::init::setup_connection;
use anyhow::{anyhow, Result};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::oneshot;

pub enum Task {
    SetupConnection(PathBuf, oneshot::Sender<Result<()>>),
    SetupChainClient(oneshot::Sender<Result<()>>),
    SetupNodeClient(oneshot::Sender<Result<()>>),
}

pub async fn run_task(task: Task, shared_state: Arc<Clients>) -> Result<()> {
    match task {
        Task::SetupConnection(path, response) => {
            let (init_client, thread_client) = match setup_connection(path.as_ref()).await {
                Ok(clients) => clients,
                Err(e) => {
                    let err = anyhow!("Failed to setup connection: {}", e);
                    response.send(Err(err)).unwrap_or(());
                    return Err(anyhow!("Failed to setup connection"));
                }
            };

            {
                let mut init_lock = shared_state.init_client.write().unwrap();
                let mut thread_lock = shared_state.thread_client.write().unwrap();
                *init_lock = Some(init_client);
                *thread_lock = Some(thread_client);
            }

            response.send(Ok(())).unwrap_or(());
        }
        _ => unimplemented!(),
    }
    Ok(())
}
