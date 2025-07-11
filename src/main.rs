use std::error::Error;

mod app;
mod config;

use app::create_app;
use config::get_config;
use dotenv::dotenv;
use opentelemetry::trace::TracerProvider;
use opentelemetry_otlp::SpanExporter;
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::trace::SdkTracerProvider;
use tokio::net::TcpListener;
use tracing::{info, span};
use tracing_subscriber::Registry;
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let resource = Resource::builder()
        .with_service_name("axum_starter")
        .build();
    let exporter = SpanExporter::builder().with_http().build()?;

    let provider = SdkTracerProvider::builder()
        .with_batch_exporter(exporter)
        .with_resource(resource)
        .build();

    let tracer = provider.tracer("axum_starter_tracker");
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    if cfg!(debug_assertions) {
        let fmt_layer = tracing_subscriber::fmt::layer()
            .with_target(false)
            .with_thread_ids(true)
            .with_line_number(true)
            .with_level(true);

        let subscriber = Registry::default().with(telemetry).with(fmt_layer);
        tracing::subscriber::set_global_default(subscriber)?;
    } else {
        let subscriber = Registry::default().with(telemetry);
        tracing::subscriber::set_global_default(subscriber)?;
    }

    let root = span!(tracing::Level::TRACE, "app_start", work_units = 2);
    let _enter = root.enter();
    info!("server is running in {}", get_config().address);

    let app = create_app().await?;
    let listener = TcpListener::bind(get_config().address.as_str()).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
