use thiserror::Error;

#[derive(Debug, Error)]
pub enum KeyGenerationError {
    #[error("Failed to generate random bytes")]
    RandomBytesError,

    #[error("Failed to generate private key")]
    SecretKeyError,
}