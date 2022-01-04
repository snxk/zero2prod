use sqlx::PgPool;
use std::net::TcpListener;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

// TODO - Add CI/CD support for Rust and Postgres
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    LogTracer::init().expect("Failed to initialize logger");

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new("zero2prod".into(), std::io::stdout);
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    set_global_default(subscriber).expect("Failed to set global default subscriber");

    let config = get_configuration().expect("Failed to load configuration");
    let connection_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to database");
    let listener = TcpListener::bind(&format!("127.0.0.1:{}", config.application_port))
        .expect("Failed to bind");
    let address = listener.local_addr().expect("Failed to get local address");
    println!("Listening on {}", address);

    run(listener, connection_pool)?.await
}
