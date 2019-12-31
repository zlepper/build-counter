use crate::utils::*;
use actix_http::error::ParseError;
use actix_http::http::header::ContentType;
use actix_http::http::StatusCode;
use actix_http::Response;
use actix_web::body::Body;
use actix_web::error::ErrorInternalServerError;
use actix_web::{Error, HttpRequest, Responder};
use futures::future::{err, ok, Ready};
use reqwest::header::HeaderValue;

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

impl Responder for Errors {
    type Error = Error;
    type Future = Ready<Result<Response, Self::Error>>;

    fn respond_to(self, req: &HttpRequest) -> Self::Future {
        info!("Handling request error: {:?}", self);
        let res_body = match self {
            Errors::BadRequest(message) => ErrorResponse::new(400, message),
            Errors::InternalError(message) => ErrorResponse::new(500, message),
            Errors::Unauthorized => ErrorResponse::new(401, "Unauthorized".to_string()),
            Errors::NotFound => ErrorResponse::new(404, "Not found".to_string()),
            Errors::Forbidden => ErrorResponse::new(403, "Forbidden".to_string()),
        };

        let accepted = req
            .headers()
            .get("Accept")
            .unwrap_or("application/json".into())
            .to_str()
            .unwrap_or("application/json")
            .split(",")
            .take(1)
            .next()
            .unwrap()
            .split("/")
            .skip(1)
            .next()
            .ok_or(ParseError::Header)?;

        debug!("Request accepts: {}", accepted);

        ok(Response::build(
            StatusCode::from_u16(res_body.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
        )
        .content_type(ContentType::json())
        .json(res_body))
    }
}
