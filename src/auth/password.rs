use crate::errors::{AppError, Result};
use bcrypt::{DEFAULT_COST, hash, verify};

pub struct PasswordService;

impl PasswordService {
    pub fn hash_password(password: &str) -> Result<String> {
        hash(password, DEFAULT_COST)
            .map_err(|_| AppError::Internal("Failed to hash password".to_string()))
    }
    pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
        verify(password, hash)
            .map_err(|_| AppError::Internal("Failed to verify password".to_string()))
    }
}
