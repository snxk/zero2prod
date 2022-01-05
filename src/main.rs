use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

// TODO - Add CI/CD support for Rust and Postgres

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = get_configuration().expect("Failed to load configuration");

    let connection_pool = PgPoolOptions::new()
        .connect_lazy(&config.database.connection_string())
        .expect("Failed to connect to database");

    let address = format!("{}:{}", config.application.host, config.application.port);

    let listener = TcpListener::bind(address).expect("Failed to bind");

    run(listener, connection_pool)?.await
}
