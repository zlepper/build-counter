use std::ops::Deref;

use rand::{RngCore, SeedableRng};

use crate::db::system_data::{RealSystemDataRepository, SystemDataRepository};
use crate::main_db_conn::MainDbPool;
use crate::models::SystemData;
use crate::utils::*;
use std::marker::PhantomData;

const JWT_SECRET_KEY: &str = "jwt_secret";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SecretStorage<T> {
    data: Vec<u8>,
    _d: PhantomData<T>,
}

impl<T> Deref for SecretStorage<T> {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> SecretStorage<T> {
    pub fn get_or_create_secret(
        db_pool: &MainDbPool,
        name: &str,
        size: usize,
    ) -> Result<SecretStorage<T>, String> {
        let conn = db_pool.get().to_err_string()?;

        let repo = RealSystemDataRepository::new(conn);

        let existing_key = repo.get(name);

        let secret = match existing_key {
            Err(e) => {
                error!("Failed to get existing secret key: {}", e);
                return Err(format!("Failed to get existing secret key: {}", e));
            }
            Ok(None) => {
                // Create new
                let bytes = SecretStorage::generate_secret(size);

                let insert_result = repo.insert(&SystemData {
                    content: bytes.clone(),
                    key: name.to_string(),
                });

                if let Err(e) = insert_result {
                    error!("Failed to insert new secret: {}", e);
                    return Err(format!("Failed to insert new secret: {}", e));
                } else {
                    bytes
                }
            }
            Ok(Some(data)) => data.content,
        };

        Ok(SecretStorage {
            data: secret,
            _d: PhantomData,
        })
    }

    pub fn generate_secret(size: usize) -> Vec<u8> {
        let mut rng = rand::rngs::StdRng::from_entropy();
        let mut bytes = Vec::with_capacity(size);
        rng.fill_bytes(&mut bytes);

        bytes
    }
}
