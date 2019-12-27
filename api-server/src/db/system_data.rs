use diesel::prelude::*;
use rocket::request::FromRequest;
use rocket::{http::Status, Outcome, Request};

use api_server_macros::Dependency;

use crate::main_db_conn::MainDbConn;
use crate::models::SystemData;
use crate::utils::*;

pub trait SystemDataRepository {
    fn get(&self, key: &str) -> Result<Option<SystemData>, String>;
    fn insert(&self, data: &SystemData) -> Result<(), String>;
    fn delete(&self, key: &str) -> Result<(), String>;
}

impl<'a, 'r> FromRequest<'a, 'r> for Box<dyn SystemDataRepository> {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, (Status, Self::Error), ()> {
        let instance = request.guard::<RealSystemDataRepository>()?;

        Outcome::Success(Box::new(instance))
    }
}

#[derive(Dependency)]
pub struct RealSystemDataRepository {
    conn: MainDbConn,
}

impl RealSystemDataRepository {
    pub fn new(conn: MainDbConn) -> Self {
        RealSystemDataRepository { conn }
    }
}

impl SystemDataRepository for RealSystemDataRepository {
    fn get(&self, key: &str) -> Result<Option<SystemData>, String> {
        crate::schema::system_data::dsl::system_data
            .filter(crate::schema::system_data::dsl::key.eq(key))
            .first(&*self.conn)
            .to_optional()
            .to_err_string()
    }

    fn insert(&self, data: &SystemData) -> Result<(), String> {
        diesel::insert_into(crate::schema::system_data::table)
            .values(data)
            .on_conflict(crate::schema::system_data::dsl::key)
            .do_update()
            .set(data)
            .execute(&*self.conn)
            .to_err_string()
            .map(|_| ())
    }

    fn delete(&self, key: &str) -> Result<(), String> {
        diesel::delete(
            crate::schema::system_data::dsl::system_data
                .filter(crate::schema::system_data::dsl::key.eq(key)),
        )
        .execute(&*self.conn)
        .to_err_string()
        .map(|_| ())
    }
}
