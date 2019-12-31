use crate::utils::ToErrString;
use config::{Config, Environment, File};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Configuration {
    pub frontend_url: String,
    pub port: u16,
    pub database_url: String,
    pub github_client_id: String,
    pub github_client_secret: String,
    pub hostname: String,
    pub host_binding: String,
}

impl Configuration {
    pub fn get_config() -> Result<Self, String> {
        Config::default()
            .merge(File::with_name("build_counter"))
            .to_err_string()?
            .merge(Environment::with_prefix("BC"))
            .to_err_string()?
            .try_into()?
    }
}
