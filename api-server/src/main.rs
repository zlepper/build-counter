#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;

use crate::github_client_info::GitHubClientInfoFairing;
use crate::user_management::UserManagementMount;
use rocket::{Rocket};
use rocket_contrib::database;

mod db;
mod error_response;
mod github_client_info;
mod models;
mod schema;
mod session;
mod user_management;
mod utils;

#[database("main_db")]
pub struct MainDbConn(diesel::PgConnection);

fn get_rocket() -> Rocket {
    rocket::ignite()
        .attach(MainDbConn::fairing())
        .attach(GitHubClientInfoFairing)
        .mount_user_management()
}

embed_migrations!("migrations");

fn main() {
    env_logger::init();

    let r = get_rocket();

    info!("Running migrations");
    let conn = MainDbConn::get_one(&r).expect("Failed to get connection instance");
    if let Err(e) = embedded_migrations::run(&*conn) {
        panic!("Failed to urn migrations: {}", e);
    }


    info!("Starting rocket!");
    r.launch();
}
