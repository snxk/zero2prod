use zero2prod::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("Failed to bind");
    let addr = listener.local_addr().expect("Failed to get local address");
    println!("Listening on {}", addr);
    run(listener)?.await
}
