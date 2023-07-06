use axum::http::StatusCode;
use axum::{extract::State, Json};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
};

use crate::database::users::{self, Entity as Users};
use crate::routes::users::{convert_active_to_model, ResponseUser};
use crate::utils::app_error::AppError;
use crate::utils::hash::verify_password;
use crate::utils::jwt::create_token;
use crate::utils::token_wrapper::TokenWrapper;

use super::{RequestCreateUser, ResponseDataUser};

pub async fn login(
    State(db): State<DatabaseConnection>,
    State(token_secret): State<TokenWrapper>,
    Json(request_user): Json<RequestCreateUser>,
) -> Result<Json<ResponseDataUser>, AppError> {
    let user = Users::find()
        .filter(users::Column::Username.eq(request_user.username))
        .one(&db)
        .await
        .map_err(|error| {
            eprintln!("Error getting user for logging in: {:?}", error);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error logging in , please try again later",
            )
        })?;
    if let Some(user) = user {
        if !verify_password(&request_user.password, &user.password)? {
            return Err(AppError::new(
                StatusCode::UNAUTHORIZED,
                "Incorrect username and/or password",
            ));
        }
        let token = create_token(&token_secret.0, user.username.clone())?;
        let mut user = user.into_active_model();
        user.token = Set(Some(token));
        let user = user.save(&db).await.map_err(|error| {
            eprintln!("Error saving token to user in db {:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error logging you in")
        })?;
        let user = convert_active_to_model(user)?;

        let response = ResponseUser {
            id: user.id,
            username: user.username,
            token: user.token.unwrap(),
        };

        Ok(Json(ResponseDataUser { data: response }))
    } else {
        Err(AppError::new(
            StatusCode::BAD_REQUEST,
            "Incorrect username and/or password",
        ))
    }
}
