use super::app_error::AppError;
use axum::http::StatusCode;
use bcrypt::{hash, verify, DEFAULT_COST};

pub fn hash_password(password: &str) -> Result<String, AppError> {
    hash(password, DEFAULT_COST).map_err(|error| {
        eprintln!("Error hashing password: {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error securing password")
    })
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    verify(password, hash).map_err(|error| {
        eprintln!("Error verifying password: {:?}", error);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "There as a problem verifying your password",
        )
    })
}
