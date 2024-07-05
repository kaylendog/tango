use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

/// The root config instance.
#[derive(Serialize, Deserialize, Clone, Debug)]
struct Config {
    /// Module configuration.
    module: Option<Module>,
    /// A map of hostnames to host configuration.
    hosts: HashMap<String, Host>,
    /// The fallback host.
    fallback: Option<Host>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Module {
    /// The name of this module.
    name: String,
    /// A description of this module.
    description: String,
    /// This module's imports.
    imports: Vec<String>,
}

/// A host defined by the config.
#[derive(Serialize, Deserialize, Clone, Debug)]
struct Host {
    /// The port to listen on.
    port: u16,
    /// SSL configuration.
    ssl: HostSslConfiguration,
    /// A list of endpoints for this host.
    endpoints: Vec<HostEndpoint>,
}

/// A host location
#[derive(Serialize, Deserialize, Clone, Debug)]
struct HostEndpoint {
    path: Option<String>,
}

/// SSL configuration for a host.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HostSslConfiguration {
    /// Whether to enable SSL.
    enable: bool,
    /// The location of the SSL certificate.
    certificate_path: Option<PathBuf>,
    /// The location of the SSL key.
    key_path: Option<PathBuf>,
}
