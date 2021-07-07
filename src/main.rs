#![deny(unsafe_code)]

pub mod crypto;
pub mod models;

mod config;
mod auth_result;
mod routes;

pub use auth_result::*;

use actix_web::{App, HttpServer};
use config::Config;
use sqlx::postgres::PgPoolOptions;
use std::{fmt, process};

/// Displays given error to `stderr` and exits
fn err_exit(msg: impl fmt::Display) -> ! {
    eprintln!("❌ {}", msg);
    process::exit(1)
}

/// Get local package version
#[macro_export]
macro_rules! crate_version {
    () => {
        env!("CARGO_PKG_VERSION")
    };
}

/// Shows message dependant upon the encryption type of the [Config::db_url] element
fn db_is_encrypted(config: &Config) -> &str {
    match config.db_url.contains("ssl=true") {
        true => "encrypted",
        false => "\x1b[31;1;4mUNENCRYPTED\x1b[0m",
    }
}

#[tokio::main]
async fn main() {
    // config setuo
    println!("🔗 Pulling configurations..");
    dotenv::dotenv().ok();
    let config = Config::new().map_err(|err| err_exit(err)).unwrap();

    // sqlx setup
    println!("🔗 Connecting to {} database..", db_is_encrypted(&config));
    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.db_url)
        .await
    {
        Ok(val) => val,
        Err(err) => err_exit(format!("Database could not be loaded, {:?}", err)),
    };

    // run server
    println!("🚀 Starting on http://{} address!", config.hostname());
    let app_config = config.clone();
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .data(app_config.clone())
            .configure(routes::init)
    })
    .bind(config.hostname())
    .expect("❌ Could not bind to url, is it in use?") // TODO: better error
    .run()
    .await
    .expect("❌ Failed to launch app") // TODO: better error
}
