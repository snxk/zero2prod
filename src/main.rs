use std::net::SocketAddr;
use zero2prod::run;

// TODO - Implement Integration Tests

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    run(SocketAddr::from(([127, 0, 0, 1], 8000)))?.await
}
