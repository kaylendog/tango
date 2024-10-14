use std::{convert::Infallible, fs::File, net::SocketAddr, sync::Arc};

use anyhow::Error;
use daemonize::{Daemonize, Outcome, Parent};
use http_body_util::Full;
use hyper::{body::Bytes, server::conn::http1, service::service_fn, Request, Response};
use hyper_util::rt::TokioIo;
use nng::{Protocol, Socket};
use tokio::{net::TcpListener, sync::RwLock};

/// The entrypoint to the server.
pub fn start() -> Result<Parent, Error> {
    let logfile_stdout = File::create("/tmp/tango.log").unwrap();
    let logfile_stderr = logfile_stdout.try_clone()?;

    let daemonize = Daemonize::new()
        .pid_file("/tmp/tango.pid")
        .working_directory("/tmp/tango")
        .stdout(logfile_stdout)
        .stderr(logfile_stderr);

    // return on parent fork
    let _ = match daemonize.execute() {
        Outcome::Parent(res) => return Ok(res?),
        Outcome::Child(res) => match res {
            Ok(child) => child,
            Err(e) => panic!("{}", e),
        },
    };

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("failed to initialise tokio runtime");

    // "handle" errors
    if let Err(e) = rt.block_on(entrypoint()) {
        panic!("{}", e)
    };

    unreachable!("end of daemonisation")
}

/// The async entrypoint, called after the Tokio runtime has been initialised.
async fn entrypoint() -> Result<(), Error> {
    // setup control socket
    let server = Socket::new(Protocol::Req0)?;

    server.dial("");

    // bind server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;

    // config rwlock - used to prevent config changes until all requests are done serving
    let lock = Arc::new(RwLock::new(()));

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        let lock = lock.clone();

        tokio::task::spawn(async move {
            // take out read lock to prevent config changes
            let lock = lock.read().await;

            if let Err(err) = http1::Builder::new()
                // `service_fn` converts our function in a `Service`
                .serve_connection(io, service_fn(process_incoming))
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }

            // release read lock
            drop(lock);
        });
    }
}

async fn process_incoming(
    request: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("hi"))))
}
