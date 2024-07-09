//! Server implementation for the control service.

use std::net::SocketAddr;

use anyhow::Result;
use futures::{future, Future, StreamExt};
use tarpc::{
    context::Context,
    server::{self, Channel},
};

use super::{Control, IPV4_SOCKET, IPV6_SOCKET, UNIX_SOCKET};

#[derive(Clone)]
pub struct ControlServer;

impl Control for ControlServer {
    async fn start(self, ctx: Context) -> Result<(), ()> {
        todo!()
    }

    async fn stop(self, ctx: Context) -> Result<(), ()> {
        todo!()
    }

    async fn restart(self, ctx: Context) -> Result<(), ()> {
        todo!()
    }

    async fn config(self, ctx: Context) -> Result<(), ()> {
        todo!()
    }

    async fn status(self, ctx: Context) -> Result<(), ()> {
        todo!()
    }
}

/// Spawn a future on the Tokio runtime.
async fn spawn(fut: impl Future<Output = ()> + Send + 'static) {
    tokio::spawn(fut);
}

/// Bind a control server to a Unix domain socket.
pub async fn bind_unix() -> Result<impl Future> {
    // bind listener
    let listener = tarpc::serde_transport::unix::listen(
        UNIX_SOCKET,
        tarpc::tokio_serde::formats::Bincode::default,
    )
    .await?;

    Ok(listener
        .filter_map(|r| future::ready(r.ok()))
        .map(server::BaseChannel::with_defaults)
        .map(|channel| {
            let server = ControlServer;
            channel.execute(server.serve()).for_each(spawn)
        })
        .buffer_unordered(10)
        .for_each(|_| async {}))
}

/// Bind a control server to an IPv4 socket.
pub async fn bind_ipv4() -> Result<impl Future> {
    let listener = tarpc::serde_transport::tcp::listen(
        IPV4_SOCKET.parse::<SocketAddr>()?,
        tarpc::tokio_serde::formats::Bincode::default,
    )
    .await?;

    Ok(listener
        .filter_map(|r| future::ready(r.ok()))
        .map(server::BaseChannel::with_defaults)
        .map(|channel| {
            let server = ControlServer;
            channel.execute(server.serve()).for_each(spawn)
        })
        .buffer_unordered(10)
        .for_each(|_| async {}))
}

/// Bind a control server to an IPv6 socket.
pub async fn bind_ipv6() -> Result<impl Future> {
    let listener = tarpc::serde_transport::tcp::listen(
        IPV6_SOCKET.parse::<SocketAddr>()?,
        tarpc::tokio_serde::formats::Bincode::default,
    )
    .await?;

    Ok(listener
        .filter_map(|r| future::ready(r.ok()))
        .map(server::BaseChannel::with_defaults)
        .map(|channel| {
            let server = ControlServer;
            channel.execute(server.serve()).for_each(spawn)
        })
        .buffer_unordered(10)
        .for_each(|_| async {}))
}
