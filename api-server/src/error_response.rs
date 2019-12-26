use crate::utils::ToOk;
use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket::{response, Request, Response};
use std::io::Cursor;

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
    fn respond_to(self, _: &Request) -> response::Result<'r> {
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
