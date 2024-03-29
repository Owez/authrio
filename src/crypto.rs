//! Contains cryptography and random generators for use in password hashing and oauth

use crate::Config;
use chrono::prelude::*;
use rand::prelude::*;

/// Length of randomly generated salts
const SALT_LENGTH: usize = 8;

/// Length of randomly generated tokens
const TOKEN_LENGTH: usize = 32;

/// Maximum length for passwords
const MAX_PASSWORD: usize = 72;

/// Generates a random token
pub fn gen_token() -> String {
    base64::encode(rand::thread_rng().gen::<[u8; TOKEN_LENGTH]>())
}

/// Generates ids for i32 length
pub fn gen_id() -> i32 {
    rand::thread_rng().gen()
}

/// Hash container, allowing easy password hashing access
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Hash {
    /// Actual hash
    pub inner: Vec<u8>,
    /// Salt used to construct the hash
    pub salt: [u8; SALT_LENGTH],
    /// Timestamp of creation
    pub created: DateTime<Utc>,
}

impl Hash {
    /// Creates a new [struct@Hash] from salt, pepper and a given input
    pub fn new(
        config: &Config,
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
            created: Utc::now(),
        })
    }

    /// Creates a new [struct@Hash] from a plaintext password
    pub fn from_password(
        config: &Config,
        password: impl AsRef<[u8]>,
    ) -> Result<Self, argon2::Error> {
        if password.as_ref().len() > MAX_PASSWORD {
            Err(argon2::Error::PwdTooLong)
        } else {
            Self::new(config, password, gen_salt())
        }
    }

    /// Compares a given input to existing hash on record
    pub fn compare(&self, input: impl AsRef<[u8]>) -> Result<bool, argon2::Error> {
        argon2::verify_raw(
            self.inner.as_slice(),
            &self.salt[..],
            input.as_ref(),
            &argon2::Config::default(),
        )
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
fn concat_pepper<'a>(config: &Config, salt: [u8; SALT_LENGTH]) -> Vec<u8> {
    [&salt[..], config.pepper.as_slice()].concat()
}

/// Generates a new random salt, used in conjunction with [concat_pepper] to add peppering
fn gen_salt() -> [u8; SALT_LENGTH] {
    rand::thread_rng().gen()
}
