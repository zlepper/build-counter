use crate::config::Configuration;
use crate::utils::{ToErr, ToErrString, ToOk};
use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHubUser {
    pub id: i32,
    pub login: String,
    pub email: Option<String>,
    pub avatar_url: String,
    pub name: String,
}

pub struct GitHubUserOrg {}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHubResponseError {
    pub message: String,
}

impl std::fmt::Display for GitHubResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.message)
    }
}

#[derive(Clone)]
pub struct GitHubClientInfo {
    pub oauth_client: BasicClient,
}

impl GitHubClientInfo {
    pub fn get_user_info(&self, auth_token: &str) -> Result<GitHubUser, String> {
        let client = reqwest::Client::new();

        let mut response = client
            .get("https://api.github.com/user")
            .bearer_auth(auth_token)
            .send()
            .map_err(|e| {
                error!("Failed to request current user: {}", e);
                e.to_string()
            })?;

        if response.status().is_success() {
            let content = response.text().to_err_string()?;
            debug!("Github api response: {}", content);
            serde_json::from_str(&content).map_err(|e| {
                error!("Failed to deserialize current user: {}", e);
                e.to_string()
            })
        } else {
            error!("Request did not return status ok, reading er");
            response
                .json::<GitHubResponseError>()
                .map_err(|e| {
                    error!("Failed to read error response: {}", e);
                    e.to_string()
                })?
                .err()
                .map_err(|e| {
                    error!("Actual request error: {}", e);
                    e.message
                })
        }
    }

    pub fn get_user_orgs(&self, auth_token: &str) -> Result<Vec<GitHubUserOrg>, String> {
        let mut response = reqwest::Client::new()
            .get("https://api.github.com/user/orgs")
            .bearer_auth(auth_token)
            .send()
            .map_err(|e| {
                error!("Failed to request orgs for current user: {}", e);
                e.to_string()
            })?;

        let body = response.text().to_err_string()?;

        info!("User org list: {}", body);

        Ok(vec![])
    }

    pub fn get_github_client_info(cfg: &Configuration) -> Result<GitHubClientInfo, String> {
        let &Configuration {
            github_client_id,
            github_client_secret,
            port,
            hostname,
            ..
        } = cfg;

        let info = GitHubClientInfo {
            oauth_client: BasicClient::new(
                ClientId::new(github_client_id),
                Some(ClientSecret::new(github_client_secret)),
                AuthUrl::new("https://github.com/login/oauth/authorize".to_string())
                    .to_err_string()?,
                Some(
                    TokenUrl::new("https://github.com/login/oauth/access_token".to_string())
                        .to_err_string()?,
                ),
            )
            .set_redirect_url(
                RedirectUrl::new(format!("{}:{}/gh-oauth-callback", hostname, port))
                    .to_err_string()?,
            ),
        };

        Ok(info)
    }
}
