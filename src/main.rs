use actix_web::{App, HttpServer};

use dotenvy::dotenv;
use user_management::common::database::init_pool_and_execute_migrations;
use user_management::user::user_api;
use user_management::user::user_service::UserService;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set!");

    let pool = init_pool_and_execute_migrations(&database_url, 10);
    let user_service = UserService::new(pool.clone());

    HttpServer::new(move || App::new().service(user_api::init(user_service.clone())))
        .bind(("127.0.0.1", 8081))?
        .run()
        .await
}
