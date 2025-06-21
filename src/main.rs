#![allow(dead_code)]

mod app;
mod config;

use app::create_app;
use config::get_config;
use dotenv::dotenv;
use std::error::Error;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let _guard = init_tracing_opentelemetry::tracing_subscriber_ext::init_subscribers()?;

    let app = create_app().await?;
    let listener = TcpListener::bind(get_config().address.as_str()).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
