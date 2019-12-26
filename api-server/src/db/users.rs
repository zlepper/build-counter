use crate::github_client_info::GitHubUser;
use crate::models::{GitHubUserInfo, GitHubUserInfoUpdate, User};
use crate::utils::*;
use crate::MainDbConn;
use api_server_macros::Dependency;
use diesel::prelude::*;
use rocket::request::FromRequest;
use rocket::{http::Status, Outcome, Request};
use uuid::Uuid;

pub trait UserRepository {
    fn find_or_create_github_user(&self, user: GitHubUser) -> Result<User, String>;
}

impl<'a, 'r> FromRequest<'a, 'r> for Box<dyn UserRepository> {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, (Status, Self::Error), ()> {
        let instance = request.guard::<RealUserRepository>()?;

        Outcome::Success(Box::new(instance))
    }
}

#[derive(Dependency)]
pub struct RealUserRepository {
    conn: MainDbConn,
}

impl UserRepository for RealUserRepository {
    fn find_or_create_github_user(&self, user: GitHubUser) -> Result<User, String> {
        info!("Attempting to find github user: {}", user.login);
        self.conn
            .transaction(move || {
                let existing_user: Option<GitHubUserInfo> =
                    crate::schema::github_user_info::dsl::github_user_info
                        .filter(crate::schema::github_user_info::dsl::id.eq(user.id))
                        .first(&*self.conn)
                        .to_optional()?;

                match existing_user {
                    None => {
                        // Create
                        info!("GitHub user doesn't seem to be in the system, creating: {:?}", user);

                        let u = User { id: Uuid::new_v4() };
                        diesel::insert_into(crate::schema::users::table)
                            .values(&u)
                            .execute(&*self.conn)?;

                        let user_info = GitHubUserInfo {
                            id: user.id,
                            login: user.login,
                            name: user.name,
                            email: user.email,
                            avatar_url: user.avatar_url,
                            user_id: u.id.clone(),
                        };
                        let inserted_user_id =
                            diesel::insert_into(crate::schema::github_user_info::table)
                                .values(&user_info)
                                .on_conflict((crate::schema::github_user_info::dsl::id))
                                .do_update()
                                .set(&user_info.clone().into())
                                .returning(crate::schema::github_user_info::dsl::user_id)
                                .get_result(&*self.conn)?;

                        if inserted_user_id != u.id {
                            info!("Seems that the user was created by another thread already, cleaning up");
                            // The user was inserted with a different id (usually because of a race condition.
                            // Remove the user we just created
                            diesel::delete(
                                crate::schema::users::dsl::users
                                    .filter(crate::schema::users::dsl::id.eq(&u.id)),
                            )
                            .execute(&*self.conn)?;

                            Ok(User {
                                id: inserted_user_id,
                            })
                        } else {
                            info!("New system user created from github user");
                            Ok(u)
                        }
                    }
                    Some(u) => {
                        info!("User already exists, updating: {:?}", user);
                        let user_id: Uuid = diesel::update(
                            crate::schema::github_user_info::dsl::github_user_info
                                .filter(crate::schema::github_user_info::dsl::id.eq(user.id)),
                        )
                        .set(GitHubUserInfoUpdate::from(user))
                        .returning((crate::schema::github_user_info::user_id))
                        .get_result(&*self.conn)?;

                        crate::schema::users::dsl::users
                            .filter(crate::schema::users::dsl::id.eq(user_id))
                            .first(&*self.conn)
                    }
                }
            })
            .to_err_string()
    }
}
