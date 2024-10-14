use std::fs::File;

use daemonize::Daemonize;

/// The daemonised entrypoint of the server.
pub fn start() {
    let stdout = File::create("/tmp/tango.log").unwrap();

    let daemonize = Daemonize::new()
        .pid_file("/tmp/test.pid") // Every method except `new` and `start`
        .chown_pid_file(true)      // is optional, see `Daemonize` documentation
        .working_directory("/tmp") // for default behaviour.
        .user("nobody")
        .group("daemon") // Group name
        .group(2)        // or group id.
        .umask(0o777)    // Set umask, `0o027` by default.
        .stdout(stdout)  // Redirect stdout to `/tmp/daemon.out`.
        .stderr(stdout)  // Redirect stderr to `/tmp/daemon.err`.
        .privileged_action(|| "Executed before drop privileges");

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("failed to initialise tokio runtime");

    rt.block_on(async {
        // setup logging
        let tokio::fs::File::open("tango.log").await;
    })
}

/// The async entrypoint, called after the Tokio runtime has been initialised.
async fn entrypoint() {}
