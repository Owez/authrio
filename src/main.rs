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

    // POST route setup
    let auth_provider_signup = warp::path!("auth" / "provider").map(|| "hi");

    // GET route setup
    let index = warp::path::end().map(|| format!("Authrio v{}", crate_version!()));
    let auth_provider_login = warp::path!("auth" / "provider").map(|| "hi");

    // route mapping
    let post_routes = warp::post().and(auth_provider_signup);
    let get_routes = warp::get().and(index.or(auth_provider_login));
    let routes = post_routes.or(get_routes);

    // run warp
    println!("Running on {}", config.url());
    warp::serve(routes).run((config.host, config.port)).await;
}
