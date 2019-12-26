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
use crate::jwt_secret::JwtSecret;
use crate::user_management::UserManagementMount;
use crate::utils::ToOk;
use api_server_macros::InjectedResource;
use diesel::PgConnection;
use rocket::Rocket;
use rocket_contrib::database;
use std::borrow::Borrow;

mod db;
mod error_response;
mod github_client_info;
mod jwt;
mod jwt_secret;
mod models;
mod schema;
mod session;
mod user_management;
mod utils;

const FRONTEND_URL_KEY: &str = "frontend_url";

#[derive(Clone, Debug, PartialEq, Eq, InjectedResource)]
pub struct FrontendUrl(String);

impl FrontendUrl {
    fn fairing() -> impl ::rocket::fairing::Fairing {
        rocket::fairing::AdHoc::on_attach(
            "FrontendUrl config",
            |rocket| -> Result<Rocket, Rocket> {
                let frontend_url = rocket.config().get_string(FRONTEND_URL_KEY);

                match frontend_url {
                    Err(e) => {
                        error!("Failed to get config for {}: {}", FRONTEND_URL_KEY, e);
                        Err(rocket)
                    }
                    Ok(fu) => rocket.manage(FrontendUrl(fu)).ok(),
                }
            },
        )
    }
}

#[database("main_db")]
pub struct MainDbConn(diesel::PgConnection);

impl Borrow<diesel::PgConnection> for MainDbConn {
    fn borrow(&self) -> &PgConnection {
        &self.0
    }
}

fn get_rocket() -> Rocket {
    rocket::ignite()
        .attach(MainDbConn::fairing())
        .attach(GitHubClientInfoFairing)
        .attach(FrontendUrl::fairing())
        .attach(JwtSecret::fairing())
        .mount_user_management()
}

embed_migrations!("migrations");

fn main() {
    env_logger::init();

    let r = get_rocket();

    info!("Running migrations");
    let conn = MainDbConn::get_one(&r).expect("Failed to get connection instance");
    if let Err(e) = embedded_migrations::run(&*conn) {
        panic!("Failed to run migrations: {}", e);
    }

    info!("Starting rocket!");
    r.launch();
}
