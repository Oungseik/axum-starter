use crate::app::error::{AppError, AppResult, ErrorResponse};
use axum::Json;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(ToSchema, Serialize)]
pub struct UserResponse {}

#[utoipa::path(
    get,
    path = "/{id}",
    description = "",
    responses(
        (status = 200, description = "Get the user information", body = UserResponse),
        (status = 404, description = "User not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Users",
)]
#[tracing::instrument]
pub async fn get_user_by_id() -> AppResult<Json<UserResponse>> {
    // TODO show case how to use error, remove this
    Err(AppError::InternalServerError)
}
