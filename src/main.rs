#![deny(unsafe_code)]

mod config;
mod crypto;
mod models;

use config::Config;
use sqlx::postgres::PgPoolOptions;
use std::{fmt, process};
use warp::Filter;

/// Displays given error to `stderr` and exits
fn err_exit(msg: impl fmt::Display) -> ! {
    eprintln!("Error: {}", msg);
    process::exit(1)
}

#[tokio::main]
async fn main() {
    // config setuo
    dotenv::dotenv().ok();
    let config = Config::new().map_err(|err| err_exit(err)).unwrap();

    // sqlx setup
    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.db_url)
        .await
    {
        Ok(val) => val,
        Err(err) => err_exit(format!("Database could not be loaded, {:?}", err)),
    };

    // warp setup
    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name)); // example for future
    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}
