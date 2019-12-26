use crate::db::sessions::SessionRepository;
use crate::db::users::UserRepository;
use crate::error_response::Errors;
use crate::github_client_info::GitHubClientInfo;
use crate::models::{GitHubLoginSessionInformation, NewGitHubLoginSessionInformation};
use crate::schema::github_login_session_information::dsl as github_login_session_informations_dsl;
use crate::schema::github_login_session_information::table as github_login_session_information_table;
use crate::session::Session;
use crate::utils::ToInternalStatusError;
use crate::JwtSecret;
use crate::{FrontendUrl, MainDbConn};
use diesel::prelude::*;
use oauth2::reqwest::http_client;
use oauth2::{AuthorizationCode, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, TokenResponse};
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::{get, routes, Rocket};
use serde::Serialize;

pub trait UserManagementMount {
    fn mount_user_management(self) -> Self;
}

impl UserManagementMount for Rocket {
    fn mount_user_management(self) -> Self {
        self.mount("/", routes![start_login, finish_github_login])
    }
}

#[get("/start-gh-login?<return_url>")]
fn start_login(
    github_info: GitHubClientInfo,
    session: Session,
    repo: Box<dyn SessionRepository>,
    return_url: String,
    frontend_url: FrontendUrl,
) -> Result<Redirect, Errors> {
    if !return_url.starts_with(&frontend_url.0) {
        error!("Tried to request login without valid return url");
        return Err(Errors::bad_request("Invalid return_url"));
    }

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the full authorization URL.
    let (auth_url, csrf_token) = github_info
        .oauth_client
        .authorize_url(CsrfToken::new_random)
        // Set the PKCE code challenge.
        .set_pkce_challenge(pkce_challenge)
        .url();

    let session_id = session.id;

    repo.create_session_for_github_login(
        session_id,
        csrf_token.secret(),
        pkce_verifier.secret(),
        &return_url,
    )
    .to_internal_err(|e| error!("Failed to insert csrf token: {}", e))?;

    Ok(Redirect::to(auth_url.to_string()))
}

#[get("/gh-oauth-callback?<code>&<state>")]
pub fn finish_github_login(
    session: Session,
    session_repo: Box<dyn SessionRepository>,
    user_repo: Box<dyn UserRepository>,
    github_info: GitHubClientInfo,
    code: String,
    state: String,
    jwt_secret: JwtSecret,
) -> Result<Redirect, Errors> {
    info!("code: '{}', state: '{}'", code, state);

    let existing_csrf_token = session_repo
        .load_github_login_session(session.id, &state)
        .to_internal_err(|e| error!("Failed to load existing csrf token: {}", e))?;

    match existing_csrf_token {
        None => Err(Errors::bad_request("Invalid csrf token")),
        Some(token) => {
            let verifier = PkceCodeVerifier::new(token.pkce_verifier.clone());
            let token_response = github_info
                .oauth_client
                .exchange_code(AuthorizationCode::new(code))
                .set_pkce_verifier(verifier)
                .request(http_client)
                .map_err(|e| {
                    error!("Failed to exchange for github token: {}", e);
                    Errors::internal_error("GitHub connection failed")
                })?;

            session_repo
                .delete_login_session(token.id)
                .to_internal_err(|e| error!("Failed to delete user login session: {}", e))?;

            let github_user = github_info
                .get_user_info(token_response.access_token().secret())
                .to_internal_err(|s| error!("Get user info failed: {}", s))?;

            debug!("User info: {:?}", github_user);

            let system_user = user_repo
                .find_or_create_github_user(github_user)
                .to_internal_err(|e| error!("Failed to find or create github user: {}", e))?;

            debug!("Found user in system: {:?}", system_user);

            Ok(Redirect::to(token.return_url))
        }
    }
}
