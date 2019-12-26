use crate::github_client_info::GitHubClientInfo;
use crate::models::{GitHubLoginSessionInformation, NewGitHubLoginSessionInformation};
use crate::schema::github_login_session_information::dsl as github_login_session_informations_dsl;
use crate::schema::github_login_session_information::table as github_login_session_information_table;
use crate::session::Session;
use crate::MainDbConn;
use diesel::prelude::*;
use oauth2::{CsrfToken, PkceCodeChallenge};
use rocket::http::{Status};
use rocket::response::Redirect;
use rocket::{get, routes, Rocket};

pub trait UserManagementMount {
    fn mount_user_management(self) -> Self;
}

impl UserManagementMount for Rocket {
    fn mount_user_management(self) -> Self {
        self.mount("/", routes![start_login, finish_github_login])
    }
}

#[get("/start-gh-login")]
fn start_login(
    github_info: GitHubClientInfo,
    session: Session,
    conn: MainDbConn,
) -> Result<Redirect, Status> {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the full authorization URL.
    let (auth_url, csrf_token) = github_info
        .oauth_client
        .authorize_url(CsrfToken::new_random)
        // Set the PKCE code challenge.
        .set_pkce_challenge(pkce_challenge)
        .url();

    let session_id = session.id;

    let csrf_entry = NewGitHubLoginSessionInformation::new(
        session_id,
        csrf_token.secret(),
        pkce_verifier.secret(),
    );

    diesel::insert_into(github_login_session_information_table)
        .values(&csrf_entry)
        .execute(&*conn)
        .map_err(|e| {
            error!("Failed to insert csrf entry in database: {}", e);
            Status::InternalServerError
        })?;

    Ok(Redirect::to(auth_url.to_string()))
}

#[get("/gh-oauth-callback?<code>&<state>")]
pub fn finish_github_login(
    session: Session,
    conn: MainDbConn,
    code: String,
    state: String,
) -> Result<Redirect, Status> {
    info!("code: '{}', state: '{}'", code, state);

    let csrf_tokens: GitHubLoginSessionInformation =
        github_login_session_informations_dsl::github_login_session_information
            .filter(
                github_login_session_informations_dsl::session_id
                    .eq(session.id)
                    .and(github_login_session_informations_dsl::csrf_token.eq(state)),
            )
            .first(&*conn)
            .map_err(|e| {
                error!(
                    "Failed to load csrf session tokens from the database: {}",
                    e
                );
                Status::InternalServerError
            })?;

    Err(Status::InternalServerError)
}
