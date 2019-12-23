#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate diesel;

use rocket::{get, routes, Rocket};
use rocket_contrib::database;

mod models;
mod schema;

#[database("main_db")]
struct MainDbConn(diesel::PgConnection);

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
    let r = get_rocket();

    let conn = MainDbConn::get_one(&r).expect("Failed to get connection instance");

    embedded_migrations::run(&*conn);

    r.launch();
}
