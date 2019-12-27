use rocket::Rocket;

use api_server_macros::InjectedResource;

use crate::utils::*;
use std::ops::Deref;

const FRONTEND_URL_KEY: &str = "frontend_url";

#[derive(Clone, Debug, PartialEq, Eq, InjectedResource)]
pub struct FrontendUrl(String);

impl Deref for FrontendUrl {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait AttachFrontendUrlState {
    fn attach_frontend_url_state(self) -> Self;
}

impl AttachFrontendUrlState for ::rocket::Rocket {
    fn attach_frontend_url_state(self) -> Self {
        let frontend_url = self
            .config()
            .get_string(FRONTEND_URL_KEY)
            .expect("Failed to get config for frontend url");

        self.manage(FrontendUrl(frontend_url))
    }
}
