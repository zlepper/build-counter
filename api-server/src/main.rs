#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;

mod app;
mod db;
mod error_response;
mod frontend_url;
mod github_client_info;
mod jwt;
mod jwt_secret;
mod main_db_conn;
mod models;
mod schema;
mod session;
mod user_management;
mod utils;

fn main() {
    env_logger::init();
    app::start();
}
