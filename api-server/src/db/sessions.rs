use crate::models::{GitHubLoginSessionInformation, NewGitHubLoginSessionInformation};
use diesel::prelude::*;
use uuid::Uuid;
use crate::utils::*;

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
}

struct RealSessionRepository<'a> {
    conn: &'a diesel::PgConnection,
}

impl<'a> SessionRepository for RealSessionRepository<'a> {
    fn create_session_for_github_login(
        &self,
        session_id: Uuid,
        csrf_token: &str,
        pkce_verifier: &str,
    ) -> Result<(), String> {
        let info = NewGitHubLoginSessionInformation::new(session_id, csrf_token, pkce_verifier);

        diesel::insert_into(crate::schema::github_login_session_information::table)
            .values(info)
            .execute(self.conn)
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
                .first(self.conn)
                .to_optional()
                .to_err_string()
    }
}
