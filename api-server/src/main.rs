#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;

use rocket::response::Responder;
use rocket::{get, routes, Request, Response, Rocket};
use rocket_contrib::database;

mod models;
mod schema;
mod user_management;

#[database("main_db")]
pub struct MainDbConn(diesel::PgConnection);

#[get("/")]
fn index(db: MainDbConn) -> &'static str {
    "Hello, world!"
}

fn get_rocket() -> Rocket {
    rocket::ignite()
        .attach(MainDbConn::fairing())
        .mount("/", routes![index])
}

embed_migrations!("migrations");

fn main() {
    env_logger::init();

    let r = get_rocket();

    let conn = MainDbConn::get_one(&r).expect("Failed to get connection instance");

    embedded_migrations::run(&*conn);

    r.launch();
}
