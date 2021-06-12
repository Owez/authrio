//! Contains cryptography and random generators for use in password hashing and oauth

use crate::Config;
use rand::prelude::*;

/// Length of randomly generated salts
const SALT_LENGTH: usize = 8;

/// Length of randomly generated tokens
const TOKEN_LENGTH: usize = 32;

/// Generates a random token
pub fn gen_token() -> String {
    base64::encode(rand::thread_rng().gen::<[u8; TOKEN_LENGTH]>())
}

/// Hash container, allowing easy password hashing access
pub struct Hash {
    /// Actual hash
    pub inner: Vec<u8>,
    /// Salt used to construct the hash
    pub salt: [u8; SALT_LENGTH],
}

impl Hash {
    /// Creates a new [Hash] from salt, pepper and a given input
    pub fn new(
        config: Config,
        input: impl AsRef<[u8]>,
        salt: impl Into<Option<[u8; SALT_LENGTH]>>,
    ) -> Result<Self, argon2::Error> {
        let salt = match salt.into() {
            Some(v) => v,
            None => gen_salt(),
        };

        Ok(Self {
            inner: argon2::hash_raw(
                input.as_ref(),
                &concat_pepper(config, salt)[..],
                &argon2::Config::default(),
            )?,
            salt,
        })
    }
}

impl From<Hash> for Vec<u8> {
    fn from(hash: Hash) -> Self {
        hash.inner
    }
}

impl From<Hash> for [u8; SALT_LENGTH] {
    fn from(hash: Hash) -> Self {
        hash.salt
    }
}

/// Adds together a passed `salt` and a pepper from the [Config::pepper] element
fn concat_pepper<'a>(config: Config, salt: [u8; SALT_LENGTH]) -> Vec<u8> {
    [salt.to_vec(), config.pepper].concat()
}

/// Generates a new random salt, used in conjunction with [concat_pepper] to add peppering
fn gen_salt() -> [u8; SALT_LENGTH] {
    rand::thread_rng().gen()
}
