use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use zero2prod::startup::start_server;

/*
DONE - Refactor this file
TODO - Add a test for the server
TODO - Add config file
DONE - Dockerize
TODO - CI/CD
*/

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "zero2prod=debug,tower_http=debug");
    }
    tracing_subscriber::fmt::init();

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_lazy("postgres://postgres:password@localhost:5432/newsletter")
        .expect("Failed to connect to database");

    let address = SocketAddr::from(([127, 0, 0, 1], 8000)); // TODO - Read from config, 0.0.0.0 only for docker build

    start_server(address, db_pool).await
}
