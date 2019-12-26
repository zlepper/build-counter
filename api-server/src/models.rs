use crate::schema::*;
use uuid::Uuid;

#[derive(Queryable, Associations, Identifiable, Debug, Eq, PartialEq)]
pub struct Organization {
    pub id: Uuid,
    pub name: String,
}

#[derive(Queryable, Associations, Identifiable, Debug, Eq, PartialEq)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Queryable, Associations, Debug, Eq, PartialEq)]
#[belongs_to(Organization)]
#[belongs_to(User)]
pub struct OrganizationUser {
    pub user_id: Uuid,
    pub organization_id: Uuid,
}

#[derive(Queryable, Debug, Eq, PartialEq)]
pub struct GitHubLoginSessionInformation {
    pub id: Uuid,
    pub session_id: Uuid,
    pub csrf_token: String,
    pub pkce_verifier: String,
}

#[derive(Insertable)]
#[table_name = "github_login_session_information"]
pub struct NewGitHubLoginSessionInformation<'a, 'b> {
    pub id: Uuid,
    pub session_id: Uuid,
    pub csrf_token: &'a str,
    pub pkce_verifier: &'b str,
}

impl<'a, 'b> NewGitHubLoginSessionInformation<'a, 'b> {
    pub fn new(session_id: Uuid, csrf_token: &'a str, pkce_verifier: &'b str) -> Self {
        NewGitHubLoginSessionInformation {
            id: Uuid::new_v4(),
            session_id,
            csrf_token,
            pkce_verifier,
        }
    }
}
