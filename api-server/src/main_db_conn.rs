use diesel::PgConnection;
use rocket::{fairing, Rocket};
use rocket_contrib::database;
use std::borrow::Borrow;

embed_migrations!("migrations");

#[database("main_db")]
pub struct MainDbConn(diesel::PgConnection);

impl Borrow<diesel::PgConnection> for MainDbConn {
    fn borrow(&self) -> &PgConnection {
        &self.0
    }
}

impl MainDbConn {
    pub fn migration_fairing() -> impl fairing::Fairing {
        fairing::AdHoc::on_attach("migrations", |rocket| -> Result<Rocket, Rocket> {
            info!("Running migrations");
            let conn = MainDbConn::get_one(&rocket).expect("Failed to get connection instance");

            if let Err(e) = embedded_migrations::run(&*conn) {
                error!("Failed to run migrations: {}", e);
                Err(rocket)
            } else {
                Ok(rocket)
            }
        })
    }
}
