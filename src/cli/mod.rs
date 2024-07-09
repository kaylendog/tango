use std::{io, process};

use clap::Parser;
use daemonize::Daemonize;
use tracing::error;

use crate::control::{client::connect_unix, context, ControlClient};

/// Tango is a light-weight web server and reverse proxy, inspired by NGINX.
#[derive(Parser)]
struct Args {
    #[clap(flatten)]
    global_opts: GlobalOpts,
    #[clap(subcommand)]
    subcommand: Option<Subcommand>,
}

#[derive(Parser)]
struct GlobalOpts {}

#[derive(Parser)]
enum Subcommand {
    /// Start the server.
    Start(StartOpts),
    /// Stop the server.
    Stop,
    /// Restart the server.
    Restart,
    /// Configure the server.
    Config,
    /// Get the server status.
    Status,
}

#[derive(Parser)]
struct StartOpts {
    #[clap(short, long)]
    #[clap(default_value = "/tmp/tango.sock")]
    mode: String,
}

pub async fn execute() {
    // parse commands
    let Args { subcommand, .. } = Args::parse();
    let subcommand = match subcommand {
        Some(x) => x,
        None => Subcommand::Status,
    };

    match subcommand {
        Subcommand::Start(opts) => start(opts).await,
        Subcommand::Stop => stop().await,
        Subcommand::Restart => restart().await,
        Subcommand::Config => config().await,
        Subcommand::Status => status().await,
    }
}

/// Connect to the control server.
async fn connect() -> ControlClient {
    match connect_unix().await {
        Ok(client) => client,
        Err(e) => {
            match e.downcast::<io::Error>() {
                Ok(e) => match e.kind() {
                    io::ErrorKind::ConnectionRefused | io::ErrorKind::NotFound => {
                        error!("Failed to connect to Tango (connection refused). Is the server running?");
                    }
                    _ => {
                        error!("Failed to connect to Tango (I/O error): {:?}", e);
                    }
                },
                Err(e) => {
                    error!("Failed to connect to Tango (unknown error): {:?}", e);
                }
            }
            process::exit(1);
        }
    }
}

/// Start the server.
async fn start(opts: StartOpts) {
    let stdout = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("/tmp/tango.log")
        .unwrap();
    match Daemonize::new().stdout(stdout).start() {
        Ok(_) => {
            println!("Server started.");
        }
        Err(_) => todo!(),
    }
}

async fn stop() {
    // connect to the control server
    let client = connect().await;

    // start the server
    match client.stop(context()).await {
        Ok(_) => {
            println!("Server started.");
        }
        Err(_) => {
            println!("Failed to start server.");
        }
    }
}

async fn restart() {}

async fn config() {}

async fn status() {
    // connect to the control server
    let client = connect().await;

    // get the server status
    match client.status(context()).await {
        Ok(_) => {
            println!("Server is running.");
        }
        Err(_) => {
            println!("Server is not running.");
        }
    }
}
