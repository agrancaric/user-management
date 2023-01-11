use actix_web::dev::HttpServiceFactory;
use actix_web::{post, web, HttpResponse};

use crate::common::errors::UserManagmenetError;

use super::{security_jwt::encode_jwt, security_model::UserDetails};

pub fn init() -> impl HttpServiceFactory {
    web::scope("security").service(create)
}

#[post("")]
async fn create(user_details: web::Json<UserDetails>) -> Result<HttpResponse, UserManagmenetError> {
    let user_details = user_details.into_inner();
    let token = encode_jwt(&user_details)?;

    Ok(HttpResponse::Ok().json(token))
}
