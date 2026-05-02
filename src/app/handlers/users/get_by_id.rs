use std::sync::Arc;

use axum::Json;
use axum::extract::{Path, State};
use sea_orm::EntityTrait;

use crate::app::AppState;
use crate::app::error::{AppError, AppResult, ErrorResponse};

#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: String,
}

impl From<entity::user::Model> for UserResponse {
    fn from(model: entity::user::Model) -> Self {
        Self {
            id: model.id,
            username: model.username,
            email: model.email,
            created_at: model.created_at.to_string(),
        }
    }
}

#[utoipa::path(
    get,
    path = "/{id}",
    description = "Get a user by ID",
    responses(
        (status = 200, description = "Get the user information", body = UserResponse),
        (status = 404, description = "User not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Users",
)]
#[tracing::instrument]
pub async fn handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> AppResult<Json<UserResponse>> {
    let user = entity::user::Entity::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(AppError::from)?
        .ok_or(AppError::NotFound)?;

    Ok(Json(UserResponse::from(user)))
}
