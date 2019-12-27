use crate::db::system_data::{RealSystemDataRepository, SystemDataRepository};
use crate::models::SystemData;
use crate::utils::*;
use crate::MainDbConn;
use api_server_macros::InjectedResource;
use rand::{RngCore, SeedableRng};
use rocket::{fairing, Rocket};
use std::ops::Deref;

const JWT_SECRET_KEY: &str = "jwt_secret";

#[derive(Clone, Debug, PartialEq, Eq, InjectedResource)]
pub struct JwtSecret(Vec<u8>);

impl Deref for JwtSecret {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl JwtSecret {
    pub fn fairing() -> impl fairing::Fairing {
        fairing::AdHoc::on_attach("jwt secret", |rocket| -> Result<Rocket, Rocket> {
            info!("Loading jwt secret from database");
            let conn = MainDbConn::get_one(&rocket).expect("Unable to get database connection");

            let repo = RealSystemDataRepository::new(conn);

            let existing_key = repo.get(JWT_SECRET_KEY);

            let secret = match existing_key {
                Err(e) => {
                    error!("Failed to get existing jwt key: {}", e);
                    return Err(rocket);
                }
                Ok(None) => {
                    // Create new
                    let mut rng = rand::rngs::StdRng::from_entropy();
                    let mut bytes = [0; 256];
                    rng.fill_bytes(&mut bytes);

                    let byte_result: Vec<u8> = bytes.to_vec();

                    let insert_result = repo.insert(&SystemData {
                        content: byte_result.clone(),
                        key: JWT_SECRET_KEY.to_string(),
                    });

                    if let Err(e) = insert_result {
                        error!("Failed to insert new jwt secret: {}", e);
                        return Err(rocket);
                    } else {
                        byte_result
                    }
                }
                Ok(Some(data)) => data.content,
            };

            rocket.manage(JwtSecret(secret)).ok()
        })
    }
}
