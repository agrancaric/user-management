use crate::user::user_api;
use actix_web::{
    web::{self, Data},
    App, HttpServer,
};

use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;

use dotenvy::dotenv;
use user::user_service::UserService;

mod common;
mod schema;
mod user;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set!");

    let manager =
        AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
    let pool = Pool::builder(manager)
        .build()
        .expect("Unable to create connection pool");

    let user_service = UserService::new(pool);

    HttpServer::new(move || {
        App::new().service(
            web::scope("users")
                .app_data(Data::new(user_service.clone()))
                .service(user_api::get_all)
                .service(user_api::get)
                .service(user_api::create)
                .service(user_api::update),
        )
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
