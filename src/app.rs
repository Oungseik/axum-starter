mod error;
mod handlers;
mod models;

use std::sync::Arc;

use axum::Router;
use axum::http::Method;
use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};
use handlers::health;
use handlers::users;
use sqlx::SqlitePool;
use tower_http::cors::{Any, CorsLayer};
use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};
use utoipa::{Modify, OpenApi};
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;

use crate::config::get_config;

#[derive(OpenApi)]
#[openapi( modifiers(&SecurityAddon))]
pub struct ApiDoc;

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(component) = openapi.components.as_mut() {
            component.add_security_scheme(
                "Authorization",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("Authorization"))),
            );

            component.add_security_scheme(
                "auth_token",
                SecurityScheme::ApiKey(ApiKey::Cookie(ApiKeyValue::new("auth_token"))),
            );
        }
    }
}

#[derive(Clone)]
struct AppState {
    pool: SqlitePool,
}

pub async fn create_app() -> Result<Router, sqlx::Error> {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any);

    let pool = setup_db().await?;
    let state = Arc::new(AppState { pool });

    let health_route = OpenApiRouter::new().routes(routes!(health::check_health::handler));
    let users_route = OpenApiRouter::new().routes(routes!(users::get_by_id::handler));

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/health", health_route)
        .nest("/api/v1/users", users_route)
        .with_state(state)
        .split_for_parts();

    let swagger = SwaggerUi::new("/api-docs/swagger-ui").url("/api-docs/openapi.json", api);
    let router = router
        .merge(swagger)
        .layer(OtelInResponseLayer)
        .layer(OtelAxumLayer::default())
        .layer(cors);

    Ok(router)
}

pub async fn setup_db() -> Result<SqlitePool, sqlx::Error> {
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect(get_config().database_url.as_str())
        .await?;

    Ok(pool)
}
