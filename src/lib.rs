use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
use std::net::SocketAddr;

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

// We need to mark `run` as public.
// It is no longer a binary entrypoint, therefore we can mark it as async
// without having to use any proc-macro incantation.
pub fn run(address: SocketAddr) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .bind(address)?
        .run();
    // No .await here!
    Ok(server)
}
