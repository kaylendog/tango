use cli::execute;

mod cli;
mod config;
mod control;
mod server;

#[tokio::main]
async fn main() {
    execute().await
}
