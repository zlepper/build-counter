#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate actix_web;

use crate::app::StartupError;

mod api;
mod app;
mod config;
mod db;
mod error_response;
mod github_client_info;
mod jwt;
mod jwt_secret;
mod main_db_conn;
mod models;
mod redirect;
mod ruuid;
mod schema;
mod session;
mod user_management;
mod utils;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let e = app::start().await;
    match e {
        Ok(_) => Ok(()),
        Err(StartupError::ConfigError(message)) => {
            error!("Failed to read configs during startup: {}", message);
            panic!("Startup error: {}", message);
        }
        Err(StartupError::IO(err)) => Err(err),
    }
}
