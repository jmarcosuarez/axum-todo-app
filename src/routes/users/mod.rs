use axum::http::StatusCode;
use sea_orm::TryIntoModel;
use serde::{Deserialize, Serialize};

use crate::{
    database::users::{self, ActiveModel},
    utils::app_error::AppError,
};

pub mod create_user;
pub mod login;

#[derive(Serialize, Deserialize)]
pub struct ResponseDataUser {
    data: ResponseUser,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseUser {
    id: i32,
    username: String,
    token: String,
}

#[derive(Serialize, Deserialize)]
pub struct RequestCreateUser {
    username: String,
    password: String,
}

fn convert_active_to_model(user: ActiveModel) -> Result<users::Model, AppError> {
    user.try_into_model().map_err(|error| {
        eprintln!("Error saving token to user in db {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error logging you in")
    })
}
