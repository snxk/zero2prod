use actix_web::{web, HttpResponse, Responder};

#[derive(serde::Deserialize, Debug)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(form: web::Form<FormData>) -> impl Responder {
    println!("{:?}", form);
    HttpResponse::Ok().finish()
}
