//! Small static configuration structure and helper methods

use std::{env, net::SocketAddr};

/// Error whilst parsing a new [Config] structure
#[derive(Debug)]
pub(crate) enum ConfigError {
    /// [Config::host] missing
    NoHost,
    /// [Config::host] invalidly inputted and could not be parsed
    InvalidHost,
    /// [Config::port] missing
    NoPort,
    /// [Config::port] invalidly inputted and could not be parsed
    InvalidPort,
    /// [Config::pepper] missing
    NoPepper,
}

impl From<ConfigError> for &'static str {
    fn from(err: ConfigError) -> Self {
        match err {
            ConfigError::NoHost => "No host address found within environment variables",
            ConfigError::InvalidHost => "The host address given is invalid",
            ConfigError::NoPort => "No port number found within environment variables",
            ConfigError::InvalidPort => "The port number given is invalid",
            ConfigError::NoPepper => "No application pepper found within environment variables",
        }
    }
}

/// Contains basic configuration information for startup
#[derive(Debug)]
pub(crate) struct Config {
    /// Host address to show server on
    pub(crate) host: [u8; 4],
    /// Port to show server on
    pub(crate) port: u16,
    /// Cryptographic pepper to embed
    pub(crate) pepper: Vec<u8>,
}

impl Config {
    /// Creates a new [Config] from [std::env] variables found
    pub(crate) fn new() -> Result<Self, ConfigError> {
        Ok(Self {
            host: parse_host(env::var("host").map_err(|_| ConfigError::NoHost)?)?,
            port: std::env::var("port")
                .map_err(|_| ConfigError::NoPort)?
                .parse()
                .map_err(|_| ConfigError::InvalidPort)?,
            pepper: std::env::var("pepper")
                .map_err(|_| ConfigError::NoPepper)?
                .as_bytes()
                .into(),
        })
    }
}

impl From<Config> for SocketAddr {
    fn from(config: Config) -> Self {
        Self::new(config.host.into(), config.port)
    }
}

/// Parses `i.i.i.i` into a valid [Config::host] element
fn parse_host(input: String) -> Result<[u8; 4], ConfigError> {
    let mut host = [0; 4];

    for (ind, part) in input.split('.').enumerate() {
        if ind > 4 {
            return Err(ConfigError::InvalidHost);
        }

        host[ind] = part.parse().map_err(|_| ConfigError::InvalidHost)?;
    }

    Ok(host)
}
