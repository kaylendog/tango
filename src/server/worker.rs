use std::{net::SocketAddr, sync::Arc};

use anyhow::{Context, Error};
use axum::{routing::get, Router};
use tokio::{
    net::TcpListener,
    sync::{broadcast, mpsc, Mutex},
};

#[derive(Clone)]
pub enum WorkerMessage {
    /// Halt this worker.
    Halt,
}

/// A worker.
pub struct Worker {
    global_rx: broadcast::Receiver<WorkerMessage>,
    config_rx: mpsc::Receiver<WorkerMessage>,
    addr: SocketAddr,
}

impl Worker {
    /// Create a new worker.
    pub fn new(
        addr: SocketAddr,
        global_rx: broadcast::Receiver<WorkerMessage>,
    ) -> (Self, mpsc::Sender<WorkerMessage>) {
        let (config_tx, config_rx) = mpsc::channel(4);
        (
            Self {
                config_rx,
                global_rx,
                addr,
            },
            config_tx,
        )
    }

    /// Consume the worker configuration and listen. Returns the configuration channel.
    pub async fn listen(mut self) -> Result<(), Error> {
        // request mutex
        let r_mutex = Arc::new(Mutex::new(()));

        // setup the server
        let app_r_mutex = r_mutex.clone();
        let app = Router::new().nest(
            "/",
            Router::new().route(
                "/",
                get(|| async move {
                    let _ = app_r_mutex.lock().await;
                }),
            ),
        );
        let listener = TcpListener::bind(self.addr)
            .await
            .context("failed to bind listener")?;

        // config task
        let config_r_mutex = r_mutex.clone();
        tokio::task::spawn(async move {
            loop {
                match self.config_rx.recv().await {
                    Some(_) => {
                        let _ = config_r_mutex.lock().await;
                    }
                    None => todo!(),
                }
            }
        });

        // global task
        let global_r_mutex = r_mutex.clone();
        tokio::task::spawn(async move {
            loop {
                match self.global_rx.recv().await {
                    Ok(_) => {
                        let _ = global_r_mutex.lock();
                    }
                    Err(_) => todo!(),
                }
            }
        });

        axum::serve(listener, app.into_make_service()).await?;

        Ok(())
    }
}
