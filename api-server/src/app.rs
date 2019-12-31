use actix_cors::Cors;
use actix_session::CookieSession;
use actix_web::{middleware, App, HttpServer};

use crate::config::Configuration;
use crate::github_client_info::GitHubClientInfo;
use crate::jwt::Jwt;
use crate::jwt_secret::SecretStorage;
use crate::main_db_conn::{MainDbPool, MainDbPoolCtor};
use crate::utils::ToOk;
use serde::export::fmt::Error;
use serde::export::Formatter;
use std::fmt::Display;
use std::sync::Arc;

#[derive(Debug)]
pub enum StartupError {
    ConfigError(String),
    IO(std::io::Error),
}

impl From<String> for StartupError {
    fn from(s: String) -> Self {
        StartupError::ConfigError(s)
    }
}

impl From<std::io::Error> for StartupError {
    fn from(e: std::io::Error) -> Self {
        StartupError::IO(e)
    }
}

impl Display for StartupError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            StartupError::ConfigError(message) => write!(f, "Failed to get configs: {}", message),
            StartupError::IO(e) => write!(f, "IO Error: {}", e),
        }
    }
}

impl std::error::Error for StartupError {}

pub async fn start() -> Result<(), StartupError> {
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
            .data(Arc::new(jwt_secret))
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
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
    .await?
    .ok()
}
