//! Small static configuration structure and helper methods

use std::{env, fmt, net::SocketAddr};

/// Amount of parts for the [parse_hosts] function
const HOST_PART_NUM: usize = 4;

/// Error whilst parsing a new [Config] structure
#[derive(Debug, PartialEq)]
pub enum ConfigError {
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

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ConfigError::NoHost => "No host address found within environment variables",
                ConfigError::InvalidHost => "The host address given is invalid",
                ConfigError::NoPort => "No port number found within environment variables",
                ConfigError::InvalidPort => "The port number given is invalid",
                ConfigError::NoPepper => "No application pepper found within environment variables",
            }
        )
    }
}

/// Contains basic configuration information for startup
#[derive(Debug)]
pub struct Config {
    /// Host address to show server on
    pub host: [u8; 4],
    /// Port to show server on
    pub port: u16,
    /// Cryptographic pepper to embed
    pub pepper: Vec<u8>,
}

impl Config {
    /// Creates a new [Config] from [std::env] variables found
    pub fn new() -> Result<Self, ConfigError> {
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
fn parse_host(input: impl AsRef<str>) -> Result<[u8; HOST_PART_NUM], ConfigError> {
    let mut host = [0; HOST_PART_NUM];
    let splitted: Vec<&str> = input.as_ref().split('.').collect();

    if splitted.len() != HOST_PART_NUM {
        return Err(ConfigError::InvalidHost);
    }

    for (ind, part) in splitted.into_iter().enumerate() {
        host[ind] = part.parse().map_err(|_| ConfigError::InvalidHost)?;
    }

    Ok(host)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_parsing() {
        assert_eq!(parse_host("127.0.0.1"), Ok([127, 0, 0, 1]));
        assert_eq!(parse_host("0.0.0.0"), Ok([0, 0, 0, 0]));
        assert_eq!(parse_host(""), Err(ConfigError::InvalidHost));
        assert_eq!(parse_host("0.0.0"), Err(ConfigError::InvalidHost));
        assert_eq!(parse_host("999.999.999.999"), Err(ConfigError::InvalidHost));
    }
}
