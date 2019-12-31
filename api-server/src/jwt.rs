use crate::jwt_secret::SecretStorage;
use crate::models::User;
use crate::utils::*;
use actix_http::{Error, Payload};
use actix_web::{FromRequest, HttpRequest};
use futures::future::{err, ok, Ready};
use uuid::Uuid;
use actix_http::http::HeaderName;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: u64,
}

const AUTHORIZATION_HEADER_NAME: &str = "Authorization";

pub struct Jwt {
    pub user_id: Uuid,
}

impl Jwt {
    pub fn create_token_for_user(user: &User, secret: &[u8]) -> Result<String, String> {
        let exp = (std::time::SystemTime::now()
            + std::time::Duration::from_secs(60 * 60 * 24 * 31))
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
        let claims = Claims {
            sub: user.id.to_string(),
            exp,
        };

        let token = jsonwebtoken::encode(&jsonwebtoken::Header::default(), &claims, secret)
            .to_err_string()?;

        Ok(token)
    }
}

impl FromRequest for Jwt {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let jwt_secret = req.app_data::<SecretStorage<Jwt>>().expect("JWT secret was not available");

        let auth_header = req.headers().get(HeaderName::AUTHORIZATION);

        let split_header = match auth_header {
            None =>
        }

        unimplemented!()
    }
    //    type Error = Errors;
    //
    //    fn from_request(request: &'a Request<'r>) -> Outcome<Self, (Status, Self::Error), ()> {
    //        let jwt_secret: SecretStorage = request.guard().expect("JWT secret was not available");
    //
    //        let auth_header = request.headers().get_one(AUTHORIZATION_HEADER_NAME);
    //
    //        let split_header = match auth_header {
    //            None => return Outcome::Forward(()),
    //            Some(a) => a.split(" ").next_tuple(),
    //        };
    //
    //        let (scheme, token) = match split_header {
    //            None => {
    //                error!("Invalid authorization header: {}", auth_header.unwrap());
    //                return Outcome::Forward(());
    //            }
    //            Some(t) => t,
    //        };
    //        if scheme != "Bearer" {
    //            error!("Invalid auth scheme: {}", scheme);
    //            return Outcome::Forward(());
    //        }
    //
    //        let claims = jsonwebtoken::decode::<Claims>(
    //            token,
    //            &*jwt_secret,
    //            &jsonwebtoken::Validation::default(),
    //        );
    //
    //        let td = match claims {
    //            Err(e) => {
    //                error!("Failed to decode jwt token: {}", e);
    //                return Outcome::Failure((Status::Unauthorized, Errors::Unauthorized));
    //            }
    //            Ok(td) => td,
    //        };
    //
    //        let user_id = Uuid::parse_str(&td.claims.sub);
    //        match user_id {
    //            Err(e) => {
    //                error!(
    //                    "Failed to parse user id in jwt. This is a critical error: {}",
    //                    e
    //                );
    //                Outcome::Failure((Status::Unauthorized, Errors::Unauthorized))
    //            }
    //            Ok(id) => Outcome::Success(Jwt { user_id: id }),
    //        }
    //    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_token() {
        let user = User { id: Uuid::new_v4() };

        let secret = SecretStorage::generate_secret();

        let token = Jwt::create_token_for_user(&user, &secret).unwrap();

        println!("Generated token: {}", token);
    }
}
