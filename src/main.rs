use axum::{http::StatusCode, routing::get, Router, Server};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/health_check", get(health_check));

    let address = SocketAddr::from(([127, 0, 0, 1], 8000));
    Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}
