use crate::github_client_info::GitHubUser;
use crate::schema::*;
use uuid::Uuid;

#[derive(Queryable, Associations, Identifiable, Debug, Eq, PartialEq, Clone)]
pub struct Organization {
    pub id: Uuid,
    pub name: String,
}

#[derive(Queryable, Insertable, Associations, Identifiable, Debug, Eq, PartialEq, Clone)]
pub struct User {
    pub id: Uuid,
}

#[derive(Queryable, Associations, Debug, Eq, PartialEq, Clone)]
#[belongs_to(Organization)]
#[belongs_to(User)]
pub struct OrganizationUser {
    pub user_id: Uuid,
    pub organization_id: Uuid,
}

#[derive(Queryable, Debug, Eq, PartialEq, Clone)]
pub struct GitHubLoginSessionInformation {
    pub id: Uuid,
    pub session_id: Uuid,
    pub csrf_token: String,
    pub pkce_verifier: String,
    pub return_url: String,
}

#[derive(Queryable, Insertable, Debug, Eq, PartialEq, Associations, Clone)]
#[belongs_to(User)]
#[table_name = "github_user_info"]
pub struct GitHubUserInfo {
    pub id: i32,
    pub login: String,
    pub name: String,
    pub email: Option<String>,
    pub avatar_url: String,
    pub user_id: Uuid,
}

#[derive(AsChangeset)]
#[table_name = "github_user_info"]
pub struct GitHubUserInfoUpdate {
    pub login: String,
    pub name: String,
    pub email: Option<String>,
    pub avatar_url: String,
}

impl From<GitHubUserInfo> for GitHubUserInfoUpdate {
    fn from(u: GitHubUserInfo) -> Self {
        GitHubUserInfoUpdate {
            login: u.login,
            name: u.name,
            email: u.email,
            avatar_url: u.avatar_url,
        }
    }
}

impl From<GitHubUser> for GitHubUserInfoUpdate {
    fn from(u: GitHubUser) -> Self {
        GitHubUserInfoUpdate {
            login: u.login,
            name: u.name,
            email: u.email,
            avatar_url: u.avatar_url,
        }
    }
}

#[derive(Insertable)]
#[table_name = "github_login_session_information"]
pub struct NewGitHubLoginSessionInformation<'a, 'b, 'c> {
    pub id: Uuid,
    pub session_id: Uuid,
    pub csrf_token: &'a str,
    pub pkce_verifier: &'b str,
    pub return_url: &'c str,
}

impl<'a, 'b, 'c> NewGitHubLoginSessionInformation<'a, 'b, 'c> {
    pub fn new(
        session_id: Uuid,
        csrf_token: &'a str,
        pkce_verifier: &'b str,
        return_url: &'c str,
    ) -> Self {
        NewGitHubLoginSessionInformation {
            id: Uuid::new_v4(),
            session_id,
            csrf_token,
            pkce_verifier,
            return_url,
        }
    }
}
