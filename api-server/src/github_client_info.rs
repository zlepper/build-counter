use crate::utils::ToOk;
use api_server_macros::InjectedResource;
use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::request::FromRequest;
use rocket::{http::Status, Outcome, Request, Rocket, State};

#[derive(Clone, InjectedResource)]
pub struct GitHubClientInfo {
    pub oauth_client: BasicClient,
}

pub struct GitHubClientInfoFairing;

impl Fairing for GitHubClientInfoFairing {
    fn info(&self) -> Info {
        Info {
            name: "GitHubClientInfo",
            kind: Kind::Attach,
        }
    }

    fn on_attach(&self, rocket: Rocket) -> Result<Rocket, Rocket> {
        let client_id = rocket.config().get_string("github_client_id");
        let client_secret = rocket.config().get_string("github_client_secret");
        let main_host = rocket
            .config()
            .get_string("host")
            .unwrap_or("http://localhost".to_string());
        let port = rocket.config().port;

        match (client_id, client_secret) {
            (Err(e), _) => {
                error!("github_client_id was not set: {}", e);
                Err(rocket)
            }
            (_, Err(e)) => {
                error!("github_client_secret was not set: {}", e);
                Err(rocket)
            }
            (Ok(c_id), Ok(c_secret)) => rocket
                .manage(GitHubClientInfo {
                    oauth_client: BasicClient::new(
                        ClientId::new(c_id),
                        Some(ClientSecret::new(c_secret)),
                        AuthUrl::new("https://github.com/login/oauth/authorize".to_string())
                            .unwrap(),
                        Some(
                            TokenUrl::new(
                                "https://github.com/login/oauth/access_token".to_string(),
                            )
                            .unwrap(),
                        ),
                    )
                    .set_redirect_url(
                        RedirectUrl::new(format!("{}:{}/gh-oauth-callback", main_host, port))
                            .unwrap(),
                    ),
                })
                .ok(),
        }
    }
}
