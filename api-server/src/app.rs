use actix_cors::Cors;
use actix_session::CookieSession;
use actix_web::{middleware, App, HttpServer};

use crate::config::Configuration;
use crate::github_client_info::GitHubClientInfo;
use crate::jwt::Jwt;
use crate::jwt_secret::SecretStorage;
use crate::main_db_conn::{MainDbPool, MainDbPoolCtor};

pub async fn start() -> std::io::Result<()> {
    let cfg = Configuration::get_config()?;

    let db_pool = MainDbPool::get_pool(&cfg)?;

    let github_info = GitHubClientInfo::get_github_client_info(&cfg)?;

    let jwt_secret = SecretStorage::<Jwt>::get_or_create_secret(&db_pool, "jwt_secret", 256)?;

    let cookie_secret =
        SecretStorage::<CookieSession>::get_or_create_secret(&db_pool, "cookie_secret", 32)?;

    HttpServer::new(|| {
        App::new()
            .data(db_pool.clone())
            .data(github_info)
            .data(cfg)
            .data(jwt_secret)
            .wrap(CookieSession::signed(&cookie_secret).secure(false))
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::new()
                    .allowed_origin(&cfg.frontend_url)
                    .max_age(3600)
                    .finish(),
            )
    })
    .bind("127.0.0.1:9000")?
    .run()
    .await
}
