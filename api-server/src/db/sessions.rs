use crate::models::GitHubLoginSessionInformation;
use std::fmt::Debug;
use uuid::Uuid;

pub trait SessionRepository: Debug + Send + Sync {
    // Should create a new session for a github login
    fn create_session_for_github_login(
        session_id: Uuid,
        csrf_token: String,
        pkce_verifier: String,
    ) -> Result<(), String>;
    // Should attempt to load an existing session if it exists.
    // If it doesn't, the implementation should return Ok(None)
    fn load_github_login_session(
        session_id: Uuid,
        csrf_token: String,
    ) -> Result<Option<GitHubLoginSessionInformation>, String>;
}

struct RealSessionRepository;

//impl SessionRepository
