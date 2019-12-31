use crate::config::Configuration;
use crate::utils::ToErrString;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use std::borrow::Borrow;

embed_migrations!("migrations");

pub type MainDbPool = r2d2::Pool<ConnectionManager<diesel::PgConnection>>;

pub trait MainDbPoolCtor {
    fn get_pool(cfg: &Configuration) -> Result<Self, String>;
}

impl MainDbPoolCtor for MainDbPool {
    fn get_pool(cfg: &Configuration) -> Result<MainDbPool, String> {
        let manager = ConnectionManager::<diesel::PgConnection>::new(&cfg.database_url);
        let pool = r2d2::Pool::builder().build(manager).to_err_string()?;

        let conn = pool.get().to_err_string()?;

        info!("Running migrations");
        if let Err(e) = embedded_migrations::run(&conn) {
            error!("Failed to run migrations: {}", e);
            Err(format!("Failed to run migrations: {}", e))
        } else {
            drop(conn);
            Ok(pool)
        }
    }
}
