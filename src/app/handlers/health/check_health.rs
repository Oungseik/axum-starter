#[utoipa::path(
    get,
    path = "/",
    responses((status = 200, description = "server is up and running")),
    tag = "Health",
)]
#[tracing::instrument]
pub async fn handler() -> &'static str {
    "server is up and running"
}
