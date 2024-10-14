use clap::Parser;
use tracing::Level;
use tracing_subscriber::EnvFilter;

mod config;
mod control;
mod server;

/// Tango is a high-performance, human-configurable reverse proxy.
#[derive(Parser)]
struct Args {
    /// Global arguments.
    #[clap(flatten)]
    global: GlobalOpts,
    /// Subcommand.
    #[clap(subcommand)]
    command: Option<Subcommand>,
}

#[derive(Parser)]
struct GlobalOpts {}

#[derive(Parser, Default)]
enum Subcommand {
    /// Start a Tango server. By default, this starts Tango as a system daemon.
    Start(StartOpts),
    /// Return information about the currently running Tango server.
    #[default]
    Info,
}

#[derive(Parser, Default)]
struct StartOpts {}

fn main() {
    let Args { global, command } = Args::parse();

    // initialise logging
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(Level::INFO.into())
                .from_env_lossy(),
        )
        .json()
        .init();

    match command.unwrap_or_default() {
        Subcommand::Start(opts) => {
            server::start().expect("failed to start server");
            println!("Server started.");
        }
        Subcommand::Info => todo!(),
    };
}
