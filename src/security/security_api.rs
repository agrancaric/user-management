use actix_web::dev::HttpServiceFactory;
use actix_web::web::Json;
use actix_web::{post, web, HttpResponse};

use crate::common::errors::UserManagementError;

use super::{security_jwt::encode_jwt, security_model::UserDetails};

pub fn init() -> impl HttpServiceFactory {
    web::scope("security").service(create)
}

#[post("")]
async fn create(user_details: Json<UserDetails>) -> Result<HttpResponse, UserManagementError> {
    let user_details = user_details.into_inner();
    let token = encode_jwt(&user_details)?;

    Ok(HttpResponse::Ok().json(token))
}
