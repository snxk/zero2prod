use axum::{
    extract::Form,
    http::StatusCode,
    routing::{get, post},
    Router, Server,
};
use serde::Deserialize;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;

#[derive(Deserialize, Debug)]
struct FormData {
    name: String,
    email: String,
}

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "zero2prod=debug,tower_http=debug");
    }
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .layer(TraceLayer::new_for_http());

    let address = SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::info!("Listening on http://{}", address);
    Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

async fn subscribe(form: Form<FormData>) -> StatusCode {
    tracing::info!("name - {} | email - {}", form.0.name, form.0.email);
    StatusCode::OK
}
