use utoipa_axum::{router::OpenApiRouter, routes};

pub fn get_health_check_router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(check_health))
}

#[utoipa::path(
    get,
    path = "/",
    responses((status = 200, description = "server is up and running")),
    tag = "Health",
)]
#[tracing::instrument]
async fn check_health() -> &'static str {
    "server is up and running"
}
