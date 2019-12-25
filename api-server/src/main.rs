#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;

use crate::user_management::UserManagementMount;
use crate::utils::ToOk;
use oauth2::basic::BasicClient;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope,
    TokenResponse, TokenUrl,
};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::response::Responder;
use rocket::{get, routes, Request, Response, Rocket};
use rocket_contrib::database;

mod error_response;
mod models;
mod schema;
mod session;
mod user_management;
mod utils;

#[database("main_db")]
pub struct MainDbConn(diesel::PgConnection);

pub struct GitHubClientInfo {
    pub oauth_client: BasicClient,
}

struct GitHubClientInfoFairing;

impl Fairing for GitHubClientInfoFairing {
    fn info(&self) -> Info {
        Info {
            name: "GitHubClientInfo",
            kind: Kind::Attach,
        }
    }

    fn on_attach(&self, rocket: Rocket) -> Result<Rocket, Rocket> {
        let client_id = rocket.config().get_string("github_client_id");
        let client_secret = rocket.config().get_string("github_client_secret");
        let main_host = rocket
            .config()
            .get_string("host")
            .unwrap_or("http://localhost".to_string());
        let port = rocket.config().port;

        match (client_id, client_secret) {
            (Err(e), _) => {
                error!("github_client_id was not set: {}", e);
                Err(rocket)
            }
            (_, Err(e)) => {
                error!("github_client_secret was not set: {}", e);
                Err(rocket)
            }
            (Ok(c_id), Ok(c_secret)) => rocket
                .manage(GitHubClientInfo {
                    oauth_client: BasicClient::new(
                        ClientId::new(c_id),
                        Some(ClientSecret::new(c_secret)),
                        AuthUrl::new("https://github.com/login/oauth/authorize".to_string())
                            .unwrap(),
                        Some(
                            TokenUrl::new(
                                "https://github.com/login/oauth/access_token".to_string(),
                            )
                            .unwrap(),
                        ),
                    )
                    .set_redirect_url(
                        RedirectUrl::new(format!("{}:{}/gh-oauth-callback", main_host, port))
                            .unwrap(),
                    ),
                })
                .ok(),
        }
    }
}

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
    embedded_migrations::run(&*conn);

    info!("Starting rocket!");
    r.launch();
}
