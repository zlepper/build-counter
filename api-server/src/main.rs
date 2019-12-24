#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::response::Responder;
use rocket::{get, routes, Request, Response, Rocket};
use rocket_contrib::database;

mod models;
mod schema;
mod user_management;
mod utils;

#[database("main_db")]
pub struct MainDbConn(diesel::PgConnection);

pub struct GitHubClientInfo {
    pub client_id: String,
    pub client_secret: String,
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

        match (client_id, client_secret) {
            (Err(e), _) => {
                error!("github_client_id was not set: {}", e);
                Err(rocket)
            }
            (_, Err(e)) => {
                error!("github_client_secret was not set: {}", e);
                Err(rocket)
            }
            (Ok(c_id), Ok(c_secret)) => Ok(rocket.manage(GitHubClientInfo {
                client_id: c_id,
                client_secret: c_secret,
            })),
        }
    }
}

#[get("/")]
fn index(db: MainDbConn) -> &'static str {
    "Hello, world!"
}

fn get_rocket() -> Rocket {
    rocket::ignite()
        .attach(MainDbConn::fairing())
        .attach(GitHubClientInfoFairing)
        .mount("/", routes![index])
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
