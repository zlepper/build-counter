use crate::models::{GitHubLoginSessionInformation, NewGitHubLoginSessionInformation};
use crate::utils::*;
use crate::MainDbConn;
use api_server_macros::Dependency;
use diesel::prelude::*;
use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::{Outcome, Request};
use uuid::Uuid;

pub trait SessionRepository {
    // Should create a new session for a github login
    fn create_session_for_github_login(
        &self,
        session_id: Uuid,
        csrf_token: &str,
        pkce_verifier: &str,
    ) -> Result<(), String>;
    // Should attempt to load an existing session if it exists.
    // If it doesn't, the implementation should return Ok(None)
    fn load_github_login_session(
        &self,
        session_id: Uuid,
        csrf_token: &str,
    ) -> Result<Option<GitHubLoginSessionInformation>, String>;

    fn delete_login_session(&self, id: Uuid) -> Result<(), String>;
}

impl<'a, 'r> FromRequest<'a, 'r> for Box<dyn SessionRepository> {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, (Status, Self::Error), ()> {
        let instance = request.guard::<RealSessionRepository>()?;

        Outcome::Success(Box::new(instance))
    }
}

#[derive(Dependency)]
pub struct RealSessionRepository {
    conn: MainDbConn,
}

impl SessionRepository for RealSessionRepository {
    fn create_session_for_github_login(
        &self,
        session_id: Uuid,
        csrf_token: &str,
        pkce_verifier: &str,
    ) -> Result<(), String> {
        let info = NewGitHubLoginSessionInformation::new(session_id, csrf_token, pkce_verifier);

        diesel::insert_into(crate::schema::github_login_session_information::table)
            .values(info)
            .execute(&*self.conn)
            .to_err_string()?;

        Ok(())
    }

    fn load_github_login_session(
        &self,
        session_id: Uuid,
        csrf_token: &str,
    ) -> Result<Option<GitHubLoginSessionInformation>, String> {
        crate::schema::github_login_session_information::dsl::github_login_session_information
            .filter(
                crate::schema::github_login_session_information::dsl::session_id
                    .eq(session_id)
                    .and(
                        crate::schema::github_login_session_information::dsl::csrf_token
                            .eq(csrf_token),
                    ),
            )
            .first(&*self.conn)
            .to_optional()
            .to_err_string()
    }

    fn delete_login_session(&self, id: Uuid) -> Result<(), String> {
        diesel::delete(
            crate::schema::github_login_session_information::dsl::github_login_session_information
                .filter(crate::schema::github_login_session_information::dsl::id.eq(id)),
        )
        .execute(&*self.conn)
        .to_err_string()
        .map(|_| ())
    }
}
