#![deny(unsafe_code)]
#![deny(warnings)]

mod config;
mod crypto;

fn main() {
    dotenv::dotenv().ok();
    let _config = config::Config::new().unwrap();
}
