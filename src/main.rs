use sqlx::PgPool;
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
    let connection_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to database");
    let listener = TcpListener::bind(&format!("127.0.0.1:{}", config.application_port))
        .expect("Failed to bind");
    let address = listener.local_addr().expect("Failed to get local address");
    println!("Listening on {}", address);

    run(listener, connection_pool)?.await
}
