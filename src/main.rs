use zero2prod::{config::get_config, startup::start_server};

/*
DONE - Refactor this file
TODO - Add a test for the server
DONE - Add config file
TODO - Change the configparser library as is not working with docker
DONE - Dockerize
TODO - CI/CD
*/

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "zero2prod=debug,tower_http=debug");
    }
    tracing_subscriber::fmt::init();

    let config = get_config();

    start_server(config.address, config.db_pool).await
}
