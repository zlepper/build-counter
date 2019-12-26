//let exp = (std::time::SystemTime::now() + std::time::Duration::SECOND * 60 * 60 * 31).duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
//
//let claims = jwt::Claims::new(jwt::Registered {
//exp: Some(exp),
//sub: Some(system_user.id.into()),
//..Default::default()
//});
//
//let token = jwt::Token::new(jwt::header::Header {
//alg: jwt::header::Algorithm::HS256,
//typ: Some(jwt::header::HeaderType::JWT),
//kid: None,
//}, claims);
//
//token.signed(&jwt_secret.0, )

use crate::error_response::Errors;
use crate::jwt_secret::JwtSecret;
use itertools::Itertools;
use rocket::request::FromRequest;
use rocket::{http::Status, Outcome, Request};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    exp: u64,
}

const AUTHORIZATION_HEADER_NAME: &str = "Authorization";

pub struct Jwt {
    pub user_id: Uuid,
}

impl<'a, 'r> FromRequest<'a, 'r> for Jwt {
    type Error = Errors;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, (Status, Self::Error), ()> {
        let jwt_secret: JwtSecret = request.guard().expect("JWT secret was not available");

        let auth_header = request.headers().get_one(AUTHORIZATION_HEADER_NAME);

        match auth_header {
            None => Outcome::Forward(()),
            Some(a) => {
                let split_header = a.split(" ").next_tuple();

                match split_header {
                    None => {
                        error!("Invalid authorization header: {}", a);
                        Outcome::Forward(())
                    }
                    Some((scheme, token)) => {
                        if scheme != "Bearer" {
                            error!("Invalid auth scheme: {}", scheme);
                            Outcome::Forward(())
                        } else {
                            let claims = jsonwebtoken::decode::<Claims>(
                                token,
                                &*jwt_secret,
                                &jsonwebtoken::Validation::default(),
                            );

                            match claims {
                                Err(e) => {
                                    error!("Failed to decode jwt token: {}", e);
                                    Outcome::Failure((Status::Unauthorized, Errors::Unauthorized))
                                }
                                Ok(td) => {
                                    let user_id = Uuid::parse_str(&td.claims.sub);
                                    match user_id {
                                        Err(e) => {
                                            error!("Failed to parse user id in jwt. This is a critical error: {}", e);
                                            Outcome::Failure((
                                                Status::Unauthorized,
                                                Errors::Unauthorized,
                                            ))
                                        }
                                        Ok(id) => Outcome::Success(Jwt { user_id: id }),
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
