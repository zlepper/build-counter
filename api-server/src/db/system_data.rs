use diesel::prelude::*;

use crate::main_db_conn::MainDbPool;
use crate::models::SystemData;
use crate::utils::*;
use actix_web::dev::Payload;
use actix_web::Error;
use actix_web::{FromRequest, HttpRequest};
use api_server_macros::{dynamic_dependency, Dependency};
use futures::future::Ready;

#[dynamic_dependency(RealSystemDataRepository)]
pub trait SystemDataRepository {
    fn get(&self, key: &str) -> Result<Option<SystemData>, String>;
    fn insert(&self, data: &SystemData) -> Result<(), String>;
    fn delete(&self, key: &str) -> Result<(), String>;
}

#[derive(Dependency)]
pub struct RealSystemDataRepository {
    conn: MainDbPool,
}

impl RealSystemDataRepository {
    pub fn new(conn: MainDbPool) -> Self {
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
