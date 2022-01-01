use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = get_configuration().expect("Failed to load configuration");
    let listener = std::net::TcpListener::bind(&format!("127.0.0.1:{}", config.application_port))
        .expect("Failed to bind");
    let address = listener.local_addr().expect("Failed to get local address");
    println!("Listening on {}", address);
    run(listener)?.await
}
