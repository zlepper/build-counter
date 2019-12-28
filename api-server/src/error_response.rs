use crate::utils::ToOk;
use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket::{response, Request, Response};
use std::io::Cursor;

#[derive(Debug)]
pub enum Errors {
    BadRequest(String),
    InternalError(String),
    Unauthorized,
    NotFound,
    Forbidden,
}

impl Errors {
    pub fn bad_request(message: &str) -> Errors {
        Errors::BadRequest(message.to_string())
    }

    pub fn internal_error(message: &str) -> Errors {
        Errors::InternalError(message.to_string())
    }
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
        info!("Handling request error: {:?}", self);
        let res_body = match self {
            Errors::BadRequest(message) => ErrorResponse::new(400, message),
            Errors::InternalError(message) => ErrorResponse::new(500, message),
            Errors::Unauthorized => ErrorResponse::new(401, "Unauthorized".to_string()),
            Errors::NotFound => ErrorResponse::new(404, "Not found".to_string()),
            Errors::Forbidden => ErrorResponse::new(403, "Forbidden".to_string()),
        };

        let accepted = request
            .headers()
            .get("Accept")
            .next()
            .unwrap_or("application/json")
            .split(",")
            .take(1)
            .next()
            .unwrap()
            .split("/")
            .skip(1)
            .next()
            .ok_or(Status::InternalServerError)?;

        debug!("Request accepts: {}", accepted);

        let body = serde_json::to_string(&res_body);

        match body {
            Err(e) => {
                error!("Failed to serialize body to json: {}", e);
                Err(Status::InternalServerError)
            }
            Ok(b) => {
                debug!("Sending json response: {}", b);
                Response::build()
                    .status(
                        Status::from_code(res_body.status).unwrap_or(Status::InternalServerError),
                    )
                    .header(ContentType::JSON)
                    .sized_body(Cursor::new(b))
                    .finalize()
                    .ok()
            }
        }
    }
}
