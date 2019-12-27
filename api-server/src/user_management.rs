use oauth2::reqwest::http_client;
use oauth2::{AuthorizationCode, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, TokenResponse};
use rocket::response::Redirect;
use rocket::{get, routes, Rocket};
use url::Url;

use crate::db::sessions::SessionRepository;
use crate::db::users::UserRepository;
use crate::error_response::Errors;
use crate::frontend_url::FrontendUrl;
use crate::github_client_info::GitHubClientInfo;
use crate::jwt::Jwt;
use crate::jwt_secret::JwtSecret;
use crate::session::Session;
use crate::utils::{ToErrString, ToInternalStatusError};

pub trait UserManagementMount {
    fn mount_user_management(self) -> Self;
}

impl UserManagementMount for Rocket {
    fn mount_user_management(self) -> Self {
        self.mount("/", routes![start_login, finish_github_login])
    }
}

fn is_valid_return_url(frontend_url: &str, return_url: &str) -> Result<bool, String> {
    let ru = Url::parse(return_url)
        .to_err_string()
        .map_err(|e| format!("Failed to parse return url: {}", e))?;

    let fu = Url::parse(frontend_url).expect("Invalid frontend url configured");

    let result = ru.domain() == fu.domain()
        && ru.scheme() == fu.scheme()
        && ru.port_or_known_default() == fu.port_or_known_default();

    Ok(result)
}

#[get("/start-gh-login?<return_url>")]
fn start_login(
    github_info: GitHubClientInfo,
    session: Session,
    repo: Box<dyn SessionRepository>,
    return_url: String,
    frontend_url: FrontendUrl,
) -> Result<Redirect, Errors> {
    let valid_return = is_valid_return_url(&*frontend_url, &return_url).map_err(|e| {
        error!("{}", e);
        Errors::BadRequest(e)
    })?;
    if !valid_return {
        return Err(Errors::BadRequest(
            "The provided return url is not a valid redirect url".to_string(),
        ));
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

    let token = match existing_csrf_token {
        None => return Err(Errors::bad_request("Invalid csrf token")),
        Some(token) => token,
    };

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

    let jwt = Jwt::create_token_for_user(&system_user, &*jwt_secret)
        .to_err_string()
        .to_internal_err(|e| error!("Failed to generate jwt token for user: {}", e))?;

    let mut ru = Url::parse(&token.return_url)
        .to_err_string()
        .to_internal_err(|e| error!("Failed to parse return url: {}", e))?;

    ru.query_pairs_mut().append_pair("token", &jwt);

    Ok(Redirect::to(ru.into_string()))
}

#[cfg(test)]
mod tests {
    use rocket::error::RouteUriError::Uri;

    use super::*;

    #[derive(Debug)]
    struct UrlTestCase {
        frontend_url: &'static str,
        return_url: &'static str,
    }

    #[test]
    fn valid_urls() {
        let cases = vec![
            UrlTestCase {
                frontend_url: "http://localhost:4200",
                return_url: "http://localhost:4200",
            },
            UrlTestCase {
                frontend_url: "http://localhost:4200",
                return_url: "http://localhost:4200/",
            },
            UrlTestCase {
                frontend_url: "http://localhost:4200",
                return_url: "http://localhost:4200/foo",
            },
            UrlTestCase {
                frontend_url: "http://localhost:4200",
                return_url: "http://localhost:4200?foo=bar",
            },
            UrlTestCase {
                frontend_url: "http://localhost:4200",
                return_url: "http://localhost:4200?foo=bar&baz=boom",
            },
            UrlTestCase {
                frontend_url: "http://localhost:4200",
                return_url: "http://localhost:4200#fragment",
            },
            UrlTestCase {
                frontend_url: "http://localhost:4200",
                return_url: "http://localhost:4200/foo?bar=baz",
            },
            UrlTestCase {
                frontend_url: "http://my.site.com",
                return_url: "http://my.site.com",
            },
            UrlTestCase {
                frontend_url: "http://my.site.com",
                return_url: "http://my.site.com:80",
            },
            UrlTestCase {
                frontend_url: "http://my.site.com",
                return_url: "http://my.site.com/foo",
            },
            UrlTestCase {
                frontend_url: "http://my.site.com",
                return_url: "http://my.site.com?foo=bar",
            },
        ];

        for case in cases {
            let result = is_valid_return_url(case.frontend_url, case.return_url).unwrap();
            assert!(result, "case {:?} failed", case);
        }
    }

    #[test]
    fn invalid_urls() {
        let cases = vec![
            UrlTestCase {
                frontend_url: "http://localhost:4200",
                return_url: "http://localhost:4201",
            },
            UrlTestCase {
                frontend_url: "http://localhost:4200",
                return_url: "http://localhost:4199",
            },
            UrlTestCase {
                frontend_url: "http://localhost:4200",
                return_url: "http://localhost",
            },
            UrlTestCase {
                frontend_url: "http://localhost:4200",
                return_url: "https://localhost:4200",
            },
            UrlTestCase {
                frontend_url: "http://localhost:4200",
                return_url: "https://localhost:4200",
            },
            UrlTestCase {
                frontend_url: "https://localhost:4200",
                return_url: "http://localhost:4200",
            },
            UrlTestCase {
                frontend_url: "https://my.site.com",
                return_url: "http://my.site.com",
            },
            UrlTestCase {
                frontend_url: "http://my.site.com",
                return_url: "https://my.site.com",
            },
            UrlTestCase {
                frontend_url: "http://my.site.com:443",
                return_url: "https://my.site.com",
            },
            UrlTestCase {
                frontend_url: "http://my.site.com:443",
                return_url: "http://my.site.com",
            },
            UrlTestCase {
                frontend_url: "https://my.site.com:443",
                return_url: "http://my.site.com",
            },
            UrlTestCase {
                frontend_url: "https://my.site.com:80",
                return_url: "http://my.site.com",
            },
            UrlTestCase {
                frontend_url: "https://my.site.com:80",
                return_url: "https://my.site.com",
            },
            UrlTestCase {
                frontend_url: "http://my.site.com",
                return_url: "http://my.site.com.total.leet.hacker.com",
            },
        ];

        for case in cases {
            let result = is_valid_return_url(case.frontend_url, case.return_url).unwrap();
            assert!(!result, "case {:?} succeeded", case);
        }
    }
}
