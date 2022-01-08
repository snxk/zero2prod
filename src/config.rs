use configparser::ini::Ini;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::net::SocketAddr;

pub struct Config {
    pub address: SocketAddr,
    pub db_pool: Pool<Postgres>,
}

impl Config {
    fn new(address: SocketAddr, db_pool: Pool<Postgres>) -> Self {
        Self { address, db_pool }
    }
}

pub fn get_config() -> Config {
    let mut config = Ini::new();
    let environment =
        std::env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "development".to_string());

    //* Could be refactored */
    match environment.as_str() {
        "development" => config.load("config/development.ini").unwrap(),
        "production" => config.load("config/production.ini").unwrap(),
        _ => panic!("Unknown environment"),
    };

    let app_port = config
        .getuint("app", "port")
        .expect("Port not found")
        .unwrap();

    let app_host = config.get("app", "host").unwrap();

    let address = format!("{}:{}", app_host, app_port)
        .parse::<SocketAddr>()
        .unwrap();

    let db_host = config.get("database", "host").unwrap();
    let db_port = config
        .getuint("database", "port")
        .expect("Port not found")
        .unwrap();
    let db_user = config.get("database", "username").unwrap();
    let db_password = config.get("database", "password").unwrap();
    let db_name = config.get("database", "dbname").unwrap();
    let db_sslmode = config
        .getbool("database", "require_ssl")
        .expect("SSL mode not found")
        .unwrap();

    let connection_string = if db_sslmode {
        format!(
            "postgres://{}:{}@{}:{}/{}?sslmode=require",
            db_user, db_password, db_host, db_port, db_name
        )
    } else {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            db_user, db_password, db_host, db_port, db_name
        )
    };

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_lazy(&connection_string)
        .expect("Failed to connect to database");

    Config::new(address, db_pool)
}
