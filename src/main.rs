mod routes;
use actix_web::{HttpServer, App};
use routes::{get_messages, set_message, homepage};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let _redis_connection = _client.get_connection();
    HttpServer::new(|| {
        App::new()
            .service(get_messages)
            .service(set_message)
            .service(homepage)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


