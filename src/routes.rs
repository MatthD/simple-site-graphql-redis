use actix_web::{get, HttpResponse, Responder, post, web};

use redis::Commands;

use crate::AppState;

#[get("/")]
pub async fn homepage() -> impl Responder {
    println!("Someone accessed / route");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/homepage.html"))
}

#[get("/messages")]
pub async fn get_messages(redis_client: web::Data<AppState>) -> impl Responder {
  let mut conn = redis_client.redis_client.get_connection().unwrap();  
  let messages: Vec<String> = conn.lrange("messages", 0, -1).unwrap_or_else(|_| vec![String::from("Messages not found")]);

  dbg!(messages.clone());

 HttpResponse::Ok().json(messages)
}

#[post("/message")]
pub async fn set_message(redis_client: web::Data<AppState>, _message: String) -> impl Responder {
  let mut _conn = redis_client.redis_client.get_connection().unwrap();
  let _: () = _conn.lpush("messages", _message).unwrap();
  HttpResponse::Ok()
}
