use axum::{
    extract::{Extension, Form},
    http::StatusCode,
    routing::{get, post},
    AddExtensionLayer, Router, Server,
};
use chrono::Utc;
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use uuid::Uuid;

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

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost:5432/newsletter")
        .await
        .expect("Failed to connect to database");

    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .layer(AddExtensionLayer::new(db_pool))
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

async fn subscribe(form: Form<FormData>, Extension(pool): Extension<Pool<Postgres>>) -> StatusCode {
    tracing::info!("name - {} | email - {}", form.0.name, form.0.email);
    match sqlx::query!(
        r#"
            INSERT INTO subscriptions (id, name, email, subscribed_at)
            VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.0.name,
        form.0.email,
        Utc::now()
    )
    .execute(&pool)
    .await
    {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            tracing::error!("{}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
