use std::os::unix::net::UnixListener;

use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    subcommand: Option<Subcommand>,
}

#[derive(Parser)]
enum Subcommand {
    Start,
    Stop,
    Restart,
    Config,
    Status,
}

fn main() -> Result<()> {
    // parse commands
    let Args { subcommand } = Args::parse();
    let subcommand = match subcommand {
        Some(x) => x,
        None => Subcommand::Status,
    };

    // setup control socket
    let control_socket = "/tmp/tango.sock";
    let _ = std::fs::remove_file(control_socket);
    let listener = UnixListener::bind(control_socket).context("failed to bind control socket")?;

    match subcommand {
        Subcommand::Start => start(),
        Subcommand::Stop => stop(),
        Subcommand::Restart => restart(),
        Subcommand::Config => config(),
        Subcommand::Status => status(),
    }
}

fn start() -> Result<()> {
    // start the server
    Ok(())
}

fn stop() -> Result<()> {
    // stop the server
    Ok(())
}

fn restart() -> Result<()> {
    // restart the server
    Ok(())
}

fn config() -> Result<()> {
    // open the config file
    Ok(())
}

fn status() -> Result<()> {
    // get the status of the server
    Ok(())
}
