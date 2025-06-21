#![allow(dead_code)]

mod app;
mod config;

use app::create_app;
use dotenv::dotenv;
use std::error::Error;
use tokio::net::TcpListener;

const ADDRESS: &str = "0.0.0.0:8080";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let _guard = init_tracing_opentelemetry::tracing_subscriber_ext::init_subscribers()?;

    let app = create_app().await?;
    let listener = TcpListener::bind(ADDRESS).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
