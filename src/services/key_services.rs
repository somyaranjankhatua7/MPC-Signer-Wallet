use rand::{RngCore, TryRngCore, rngs::OsRng};
use secp256k1::SecretKey;
use sss_rs::basic_sharing;
use std::{error::Error, result::Result};
use hex;

use crate::errors::auth_errors::KeyGenerationError;

pub struct KeyServices;

impl KeyServices {
    pub fn generate_secret_key() -> Result<SecretKey, KeyGenerationError> {
        let mut rng = OsRng;
        let mut random_bytes = [0u8; 32];
        if rng.try_fill_bytes(&mut random_bytes).is_err() {
            return Err(KeyGenerationError::RandomBytesError);
        }
        SecretKey::from_byte_array(&random_bytes).map_err(|_| KeyGenerationError::SecretKeyError)
    }

    pub fn split_secret_key(secret_key: &[u8]) -> Result<Vec<Vec<(u8, u8)>>, basic_sharing::Error> {
        basic_sharing::from_secrets(secret_key, 3, 3, None)
    }
}
