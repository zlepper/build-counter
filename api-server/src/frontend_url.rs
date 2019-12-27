use rocket::Rocket;

use api_server_macros::InjectedResource;

use crate::utils::*;

const FRONTEND_URL_KEY: &str = "frontend_url";

#[derive(Clone, Debug, PartialEq, Eq, InjectedResource)]
pub struct FrontendUrl(String);

impl FrontendUrl {
    pub fn fairing() -> impl ::rocket::fairing::Fairing {
        rocket::fairing::AdHoc::on_attach(
            "FrontendUrl config",
            |rocket| -> Result<Rocket, Rocket> {
                let frontend_url = rocket.config().get_string(FRONTEND_URL_KEY);

                match frontend_url {
                    Err(e) => {
                        error!("Failed to get config for {}: {}", FRONTEND_URL_KEY, e);
                        Err(rocket)
                    }
                    Ok(fu) => rocket.manage(FrontendUrl(fu)).ok(),
                }
            },
        )
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
