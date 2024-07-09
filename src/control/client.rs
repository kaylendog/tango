//! Client module for the control server.

use std::net::SocketAddr;

use anyhow::Result;
use tarpc::{client, tokio_serde::formats::Bincode};

use super::{ControlClient, IPV4_SOCKET, IPV6_SOCKET, UNIX_SOCKET};

/// Connect to the Unix domain socket.
pub async fn connect_unix() -> Result<ControlClient> {
    let mut transport = tarpc::serde_transport::unix::connect(UNIX_SOCKET, Bincode::default);
    transport.config_mut().max_frame_length(usize::MAX);
    let client = ControlClient::new(client::Config::default(), transport.await?).spawn();
    Ok(client)
}

/// Connect to the IPv4 control socket.
pub async fn connect_ipv4() -> Result<ControlClient> {
    let mut transport =
        tarpc::serde_transport::tcp::connect(IPV4_SOCKET.parse::<SocketAddr>()?, Bincode::default);
    transport.config_mut().max_frame_length(usize::MAX);
    let client = ControlClient::new(client::Config::default(), transport.await?).spawn();
    Ok(client)
}

/// Connect to the IPv6 control socket.
pub async fn connect_ipv6() -> Result<ControlClient> {
    let mut transport =
        tarpc::serde_transport::tcp::connect(IPV6_SOCKET.parse::<SocketAddr>()?, Bincode::default);
    transport.config_mut().max_frame_length(usize::MAX);
    let client = ControlClient::new(client::Config::default(), transport.await?).spawn();
    Ok(client)
}
