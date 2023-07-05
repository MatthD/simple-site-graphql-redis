use actix_web::{get, HttpResponse, Responder, post};

#[get("/")]
pub async fn homepage() -> impl Responder {
    println!("Someone accessed / route");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/homepage.html"))
}

#[get("/messages")]
pub async fn get_messages() -> impl Responder {
  HttpResponse::Ok().body("messages")
}

#[post("/message")]
pub async fn set_message(_message: String) -> impl Responder {
  HttpResponse::Ok().body("messages")
}
