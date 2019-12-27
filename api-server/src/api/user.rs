use rocket::{get, routes, Rocket};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::users::UserRepository;
use crate::error_response::Errors;
use crate::ruuid::RUuid;
use crate::utils::ToInternalStatusError;

pub trait UserApiMount {
    fn mount_user_api(self) -> Self;
}

impl UserApiMount for Rocket {
    fn mount_user_api(self) -> Self {
        self.mount("/api/user", routes![get_user])
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub id: Uuid,
    pub name: String,
    pub avatar_url: String,
}

#[get("/<id>")]
fn get_user(id: RUuid, user_repo: Box<dyn UserRepository>) -> Result<Json<UserResponse>, Errors> {
    let user = user_repo
        .get_user(id.into())
        .to_internal_err(|e| error!("Failed to query for user: {}", e))?;

    match user {
        None => Err(Errors::NotFound),
        Some(u) => Ok(Json(UserResponse {
            id: u.user_id,
            name: u.name,
            avatar_url: u.avatar_url,
        })),
    }
}
