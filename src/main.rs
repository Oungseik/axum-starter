use std::error::Error;
use std::sync::Arc;

mod app;
mod config;

use app::create_app;
use config::get_config;
use opentelemetry::trace::TracerProvider;
use opentelemetry_otlp::SpanExporter;
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::trace::SdkTracerProvider;
use tokio::net::TcpListener;
use tracing::{info, span};
use tracing_subscriber::Registry;
use tracing_subscriber::prelude::*;

const SERVICE_NAME: &str = "axum_starter";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let resource = Resource::builder().with_service_name(SERVICE_NAME).build();
    let exporter = SpanExporter::builder().with_http().build()?;

    let provider = SdkTracerProvider::builder()
        .with_batch_exporter(exporter)
        .with_resource(resource)
        .build();

    #[cfg(debug_assertions)]
    {
        let fmt_layer = tracing_subscriber::fmt::layer()
            .with_target(false)
            .with_thread_ids(true)
            .with_line_number(true)
            .with_level(true);
        let tracer = provider.tracer(SERVICE_NAME);
        let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
        let subscriber = Registry::default().with(telemetry).with(fmt_layer);
        tracing::subscriber::set_global_default(subscriber)?;
    }

    #[cfg(not(debug_assertions))]
    {
        let tracer = provider.tracer(SERVICE_NAME);
        let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
        let subscriber = Registry::default().with(telemetry);
        tracing::subscriber::set_global_default(subscriber)?;
    }

    let root = span!(tracing::Level::INFO, "app_start");
    let _enter = root.enter();
    info!("server is running in {}", get_config().address);

    let app = create_app().await?;
    let listener = TcpListener::bind(get_config().address.as_str()).await?;

    let provider = Arc::new(provider);
    let shutdown_provider = provider.clone();

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    shutdown_provider.shutdown()?;
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
