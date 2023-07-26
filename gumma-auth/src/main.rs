#![allow(unused_imports)]
#![allow(dead_code)]

mod frontend;

use actix_web::{self, middleware::Logger, web};
use frontend::config_gumma_auth;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .service(web::scope("/api/v1").configure(config_gumma_auth))
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
