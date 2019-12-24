use crate::schema::*;
use crate::utils::ToOk;
use diesel::prelude::*;
use diesel::ExpressionMethods;
use rocket::http::{ContentType, Status};
use rocket::response::status::BadRequest;
use rocket::response::Responder;
use rocket::{response, Request, Response};
use rocket_contrib::json::Json;
use std::io::Cursor;
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

pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}

#[derive(Debug)]
pub enum Errors {
    BadRequest(String),
    InternalError(String),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ErrorResponse {
    pub message: String,
    pub status: u16,
}

impl ErrorResponse {
    pub fn new(status: u16, message: String) -> Self {
        ErrorResponse { message, status }
    }
}

impl<'r> Responder<'r> for Errors {
    fn respond_to(self, request: &Request) -> response::Result<'r> {
        let res_body = match self {
            Errors::BadRequest(message) => ErrorResponse::new(400, message),
            Errors::InternalError(message) => ErrorResponse::new(500, message),
        };

        let body = serde_json::to_string(&res_body);
        match body {
            Err(e) => {
                error!("Failed to serialize body to json: {}", e);
                Err(Status::InternalServerError)
            }
            Ok(b) => Response::build()
                .status(Status::from_code(res_body.status).unwrap_or(Status::InternalServerError))
                .header(ContentType::JSON)
                .sized_body(Cursor::new(b))
                .finalize()
                .ok(),
        }
    }
}
