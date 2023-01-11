use std::env;

use actix_web::{App, HttpServer};

use actix_web_httpauth::middleware::HttpAuthentication;
use dotenvy::dotenv;
use user_management::common::database::init_pool_and_execute_migrations;
use user_management::security::security_api;
use user_management::security::security_web::jwt_credentials_extractor;
use user_management::user::user_api;
use user_management::user::user_service::UserService;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
    let pool_size = env::var("DATABASE_POOL_SIZE")
        .expect("DATABASE_POOL_SIZE must be set!")
        .parse()
        .unwrap();

    let pool = init_pool_and_execute_migrations(&database_url, pool_size);
    let user_service = UserService::new(pool.clone());

    HttpServer::new(move || {
        App::new()
            .service(security_api::init())
            .service(user_api::init(user_service.clone()))
            .wrap(HttpAuthentication::with_fn(jwt_credentials_extractor))
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
