use crate::database::tasks::{self};
use crate::database::users::Model as UserModel;
use crate::queries::{task_queries, user_queries};
use crate::utils::hash::hash_password;
use crate::utils::token_wrapper::TokenWrapper;
use crate::{
    database::users,
    utils::{app_error::AppError, jwt::create_token},
};
use axum::{extract::State, Json};
use sea_orm::{DatabaseConnection, Set};

use super::{RequestCreateUser, ResponseDataUser, ResponseUser};

pub async fn create_user(
    State(db): State<DatabaseConnection>,
    State(jwt_secret): State<TokenWrapper>,
    Json(request_user): Json<RequestCreateUser>,
) -> Result<Json<ResponseDataUser>, AppError> {
    let mut new_user = users::ActiveModel {
        ..Default::default()
    };
    new_user.username = Set(request_user.username.clone());
    new_user.password = Set(hash_password(&request_user.password)?);
    new_user.token = Set(Some(create_token(&jwt_secret.0, request_user.username)?));
    let user = user_queries::save_active_user(&db, new_user).await?;
    create_default_task_for_user(&db, &user).await?;

    Ok(Json(ResponseDataUser {
        data: {
            ResponseUser {
                id: user.id,
                username: user.username,
                token: user.token.unwrap(),
            }
        },
    }))
}

async fn create_default_task_for_user(
    db: &DatabaseConnection,
    user: &UserModel,
) -> Result<(), AppError> {
    // Grab all tasks with is_default=true and insert them into db (just to let js tests pass)
    let default_task = task_queries::get_default_tasks(db).await?;

    for default_task in default_task {
        let task = tasks::ActiveModel {
            priority: Set(default_task.priority),
            title: Set(default_task.title),
            completed_at: Set(default_task.completed_at),
            description: Set(default_task.description),
            deleted_at: Set(default_task.deleted_at),
            user_id: Set(Some(user.id)),
            is_default: Set(default_task.is_default),
            ..Default::default()
        };
        task_queries::save_active_task(db, task).await?;
    }

    Ok(())
}
