#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;

use crate::frontend_url::FrontendUrl;
use crate::github_client_info::GitHubClientInfoFairing;
use crate::jwt_secret::JwtSecret;
use crate::main_db_conn::MainDbConn;
use crate::user_management::UserManagementMount;
use rocket::Rocket;

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

fn get_rocket() -> Rocket {
    rocket::ignite()
        .attach(MainDbConn::fairing())
        .attach(MainDbConn::migration_fairing())
        .attach(GitHubClientInfoFairing)
        .attach(FrontendUrl::fairing())
        .attach(JwtSecret::fairing())
        .mount_user_management()
}

fn main() {
    env_logger::init();

    let r = get_rocket();

    info!("Starting rocket!");
    r.launch();
}
