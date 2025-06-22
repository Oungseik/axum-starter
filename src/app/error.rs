use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use utoipa::ToSchema;

/// use to generate open api schema
#[derive(ToSchema, Serialize)]
pub struct ErrorResponse {
    message: String,
}

pub enum AppError {
    InternalServerError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            Self::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    message: "Internal server error".into(),
                }),
            ),
        }
        .into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
