use crate::graceful_shutdown::shutdown_signal;
use crate::routers::{health_check, subscribe};
use axum::{
    routing::{get, post},
    AddExtensionLayer, Router, Server,
};
use sqlx::{Pool, Postgres};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;

pub async fn start_server(address: SocketAddr, db_pool: Pool<Postgres>) {
    tracing::info!("{}", address);

    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .layer(AddExtensionLayer::new(db_pool))
        .layer(TraceLayer::new_for_http());

    Server::bind(&address)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap()
}
