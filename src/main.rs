#![deny(unsafe_code)]
#![deny(warnings)]

mod config;
mod crypto;

use config::Config;

fn main() {
    dotenv::dotenv().ok();
    let _config = config::Config::new().unwrap();
}
