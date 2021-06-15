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

/// Get local package version
macro_rules! crate_version {
    () => {
        env!("CARGO_PKG_VERSION")
    };
}

#[tokio::main]
async fn main() {
    // config setuo
    println!("Pulling configurations..");
    dotenv::dotenv().ok();
    let config = Config::new().map_err(|err| err_exit(err)).unwrap();

    // sqlx setup
    println!("Opening database server..");
    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.db_url)
        .await
    {
        Ok(val) => val,
        Err(err) => err_exit(format!("Database could not be loaded, {:?}", err)),
    };

    // route setup
    let index = warp::path::end().map(|| format!("Authrio v{}", crate_version!()));

    // run warp
    println!("Running on {}", config.url());
    warp::serve(index).run((config.host, config.port)).await;
}
