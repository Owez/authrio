#![deny(unsafe_code)]

mod config;
mod crypto;
mod models;

use config::Config;
use hyper::Client;
use sqlx::postgres::PgPoolOptions;
use std::{fmt, process};
use uuid::Uuid;
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
    let client = Client::new();

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
    let user_provider_post = warp::path!("user" / "p").map(|| "TODO: create new user");
    let user_provider_auth =
        warp::path!("user" / "p" / "auth").map(|| "TODO: authorise existing user");
    let user_provider_refresh =
        warp::path!("user" / "p" / "refresh").map(|| "TODO: refresh users token");

    // GET route setup
    let index = warp::path::end().map(|| format!("Authrio v{}", crate_version!()));
    let user_provider_get = warp::path!("user" / "p")
        .and(warp::query::<String>())
        .map(|_uuid| "TODO: get existing user");

    // PATCH route setup
    let user_provider_patch = warp::path!("user" / "p").map(|| "TODO: update user");

    // DELETE route setup
    let user_provider_delete = warp::path!("user" / "p").map(|| "TODO: delete user");

    // route mapping
    let post_routes = warp::post().and(
        user_provider_post
            .or(user_provider_auth)
            .or(user_provider_refresh),
    );
    let get_routes = warp::get().and(index.or(user_provider_get));
    let patch_routes = warp::patch().and(user_provider_patch);
    let delete_routes = warp::delete().and(user_provider_delete);

    // final route mapping rollup
    let routes = post_routes
        .or(get_routes)
        .or(get_routes)
        .or(patch_routes)
        .or(delete_routes);

    // run warp
    println!("Running on {}", config.url());
    warp::serve(routes).run((config.host, config.port)).await;
}
