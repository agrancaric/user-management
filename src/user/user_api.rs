use actix_web::dev::HttpServiceFactory;
use actix_web::web::Data;
use actix_web::{delete, get, post, put, web, HttpResponse};

use crate::common::errors::UserManagmenetError;
use crate::common::requests::PageRequest;
use crate::user::user_models::UserData;
use crate::user::user_request::SaveUserRequest;
use crate::user::user_service::UserService;

pub fn init(user_service: UserService) -> impl HttpServiceFactory {
    web::scope("users")
        .app_data(Data::new(user_service.clone()))
        .service(find_all)
        .service(find_by_id)
        .service(create)
        .service(update)
        .service(delete)
}

#[get("")]
async fn find_all(
    user_service: web::Data<UserService>,
    request: web::Query<PageRequest>,
) -> Result<HttpResponse, UserManagmenetError> {
    let request = request.into_inner();
    let users = user_service
        .find_all(request.offset, request.limit, request.sort_properties)
        .await?;

    Ok(HttpResponse::Ok().json(users))
}

#[get("/{id}")]
async fn find_by_id(
    user_service: web::Data<UserService>,
    id: web::Path<i32>,
) -> Result<HttpResponse, UserManagmenetError> {
    let user_id = id.into_inner();
    let user = user_service.find_by_id(user_id).await?;

    Ok(HttpResponse::Ok().json(user))
}

#[post("")]
async fn create(
    user_service: web::Data<UserService>,
    user: web::Json<SaveUserRequest>,
) -> Result<HttpResponse, UserManagmenetError> {
    let user = user.into_inner();

    let user = UserData {
        first_name: &user.first_name,
        last_name: &user.last_name,
        email: &user.email,
    };

    let user = user_service.save(user).await?;

    Ok(HttpResponse::Ok().json(user))
}

#[put("/{id}")]
async fn update(
    user_service: web::Data<UserService>,
    user: web::Json<SaveUserRequest>,
    id: web::Path<i32>,
) -> Result<HttpResponse, UserManagmenetError> {
    let user_id = id.into_inner();
    let user = user.into_inner();

    let user = UserData {
        first_name: &user.first_name,
        last_name: &user.last_name,
        email: &user.email,
    };

    let user = user_service.update(user_id, user).await?;

    Ok(HttpResponse::Ok().json(user))
}

#[delete("/{id}")]
async fn delete(
    user_service: web::Data<UserService>,
    id: web::Path<i32>,
) -> Result<HttpResponse, UserManagmenetError> {
    let user_id = id.into_inner();

    user_service.delete(user_id).await?;

    Ok(HttpResponse::Ok().json(user_id))
}
