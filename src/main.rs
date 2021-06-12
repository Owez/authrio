#![deny(unsafe_code)]

mod config;
mod crypto;
mod models;

use config::Config;
use std::{fmt, process};

fn err_exit(msg: impl fmt::Display) -> ! {
    eprintln!("Error: {}", msg);
    process::exit(1)
}

fn main() {
    dotenv::dotenv().ok();
    let _config = Config::new().map_err(|err| err_exit(err)).unwrap();
}
