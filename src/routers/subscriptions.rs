use axum::{
    extract::{Extension, Form},
    http::StatusCode,
};
use chrono::Utc;
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(
    form: Form<FormData>,
    Extension(pool): Extension<Pool<Postgres>>,
) -> StatusCode {
    tracing::info!("Adding a new subscriber");
    tracing::info!("name {} | email {}", form.0.name, form.0.email);

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
        Ok(_) => {
            tracing::info!("New Subscriber successfully added");
            StatusCode::OK
        }
        Err(e) => {
            tracing::error!("Error occured in adding a new subscriber");
            tracing::error!("{}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
