use diesel::prelude::*;
use futures::future::TryFutureExt;
use uuid::Uuid;

use api_server_macros::{dynamic_dependency, Dependency};

use crate::main_db_conn::MainDbPool;
use crate::models::{GitHubLoginSessionInformation, NewGitHubLoginSessionInformation};
use crate::utils::*;

//#[dynamic_dependency(RealSessionRepository)]
pub trait SessionRepository {
    // Should create a new session for a github login
    fn create_session_for_github_login(
        &self,
        session_id: Uuid,
        csrf_token: &str,
        pkce_verifier: &str,
        return_url: &str,
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

impl ::actix_web::FromRequest for Box<dyn SessionRepository> {
    type Error = ::actix_web::Error;
    type Future =
        ::futures::future::MapOk<::futures::future::Ready<Result<Self, Self::Error>>, ???>;
    type Config = ();

    fn from_request(
        req: &::actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        RealSessionRepository::from_request(&req, payload).map_ok(|dep| Box::new(dep))
    }
}

#[derive(Dependency)]
pub struct RealSessionRepository {
    conn: MainDbPool,
}

impl SessionRepository for RealSessionRepository {
    fn create_session_for_github_login(
        &self,
        session_id: Uuid,
        csrf_token: &str,
        pkce_verifier: &str,
        return_url: &str,
    ) -> Result<(), String> {
        let info = NewGitHubLoginSessionInformation::new(
            session_id,
            csrf_token,
            pkce_verifier,
            return_url,
        );

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
