use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;

#[derive(serde::Deserialize, Debug)]
struct FormData {
    name: String,
    email: String,
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

async fn subscribe(form: web::Form<FormData>) -> impl Responder {
    println!("{:?}", form);
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
