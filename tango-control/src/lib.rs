#[cfg(feature = "client")]
mod client;
#[cfg(feature = "client")]
pub use client::*;

#[cfg(feature = "server")]
mod server;
#[cfg(feature = "server")]
pub use server::*;

#[tarpc::service]
pub trait Control {
    /// Start the server.
    async fn start() -> Result<(), ()>;
    /// Stop the server.
    async fn stop() -> Result<(), ()>;
    /// Restart the server.
    async fn restart() -> Result<(), ()>;
    // Configure the server.
    async fn config() -> Result<(), ()>;
    /// Get the server status.
    async fn status() -> Result<(), ()>;
}

/// The path to the Unix domain socket.
pub(crate) static UNIX_SOCKET: &str = "/tmp/tango.sock";

/// The IPv4 address for the TCP control socket.
pub(crate) static IPV4_SOCKET: &str = "127.0.0.1:32100";

/// The IPv6 address for the TCP control socket.
pub(crate) static IPV6_SOCKET: &str = "[::1]:32100";

/// Re-export tarpc types.
pub mod tarpc {
    pub use tarpc::*;
}

/// Get the current context.
pub fn context() -> tarpc::context::Context {
    tarpc::context::current()
}
