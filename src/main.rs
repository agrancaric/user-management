use std::{env, io};

use actix_web::{App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use dotenvy::dotenv;

use user_management::common::database::init_pool_and_execute_migrations;
use user_management::properties::UserManagementProperties;
use user_management::security::security_api;
use user_management::security::security_web::jwt_credentials_extractor;
use user_management::user::user_api;
use user_management::user::user_service::UserService;

#[actix_web::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    dotenv().ok();

    let properties = envy::from_env::<UserManagementProperties>().unwrap();

    let pool =
        init_pool_and_execute_migrations(&properties.database_url, properties.database_pool_size);

    HttpServer::new(move || {
        App::new()
            .service(security_api::init())
            .service(user_api::init(UserService::new(pool.clone())))
            .wrap(HttpAuthentication::with_fn(jwt_credentials_extractor))
    })
    .bind((properties.server_address, properties.server_port))?
    .run()
    .await
}
