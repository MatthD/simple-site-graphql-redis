mod routes;
use redis::Client;
use actix_web::{HttpServer, App, web};

use routes::{get_messages, set_message, homepage};

pub struct AppState{
    pub redis_client: Client
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let _redis_connection: Result<redis::Connection, redis::RedisError> = _client.get_connection();
    let app_state = web::Data::new(AppState { redis_client: _client });
    HttpServer::new(move|| {
        App::new()
            .app_data(app_state.clone())
            .service(get_messages)
            .service(set_message)
            .service(homepage)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


