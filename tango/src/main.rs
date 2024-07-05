use std::path::PathBuf;

use clap::Parser;

mod worker;

/// Tango is a light-weight web server and reverse proxy, inspired by NGINX.
#[derive(Parser)]
struct Args {
    /// Path to the configuration file. If no file is found here, the defaults will be created.
    #[clap(long, default_value = "/etc/tango/config.toml")]
    config: Option<PathBuf>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
}
