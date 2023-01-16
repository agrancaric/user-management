use actix_web::dev::HttpServiceFactory;
use actix_web::web::Data;
use actix_web::{delete, get, post, put, web, HttpResponse};
use actix_web_grants::proc_macro::has_permissions;

use crate::common::errors::UserManagmenetError;
use crate::common::requests::PageRequest;

use super::user_models::UserData;
use super::user_request::SaveUserRequest;
use super::user_service::UserService;

pub fn init(user_service: UserService) -> impl HttpServiceFactory {
    web::scope("users")
        .app_data(Data::new(user_service.clone()))
        .service(find_all)
        .service(find_by_id)
        .service(save)
        .service(update)
        .service(delete)
}

#[has_permissions("UM_USER_FIND_ALL")]
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

#[has_permissions("UM_USER_FIND_BY_ID")]
#[get("/{id}")]
async fn find_by_id(
    user_service: web::Data<UserService>,
    id: web::Path<i32>,
) -> Result<HttpResponse, UserManagmenetError> {
    let user_id = id.into_inner();
    let user = user_service.find_by_id(user_id).await?;

    Ok(HttpResponse::Ok().json(user))
}

#[has_permissions("UM_USER_SAVE")]
#[post("")]
async fn save(
    user_service: web::Data<UserService>,
    user: web::Json<SaveUserRequest>,
) -> Result<HttpResponse, UserManagmenetError> {
    let user = user.into_inner();

    let user = from_request(&user);
    let user = user_service.save(&user).await?;

    Ok(HttpResponse::Ok().json(user))
}

#[has_permissions("UM_USER_UPDATE")]
#[put("/{id}")]
async fn update(
    user_service: web::Data<UserService>,
    user: web::Json<SaveUserRequest>,
    id: web::Path<i32>,
) -> Result<HttpResponse, UserManagmenetError> {
    let user_id = id.into_inner();
    let user = user.into_inner();

    let user = from_request(&user);
    let user = user_service.update(user_id, &user).await?;

    Ok(HttpResponse::Ok().json(user))
}

#[has_permissions("UM_USER_DELETE")]
#[delete("/{id}")]
async fn delete(
    user_service: web::Data<UserService>,
    id: web::Path<i32>,
) -> Result<HttpResponse, UserManagmenetError> {
    let user_id = id.into_inner();

    user_service.delete(user_id).await?;

    Ok(HttpResponse::Ok().json(user_id))
}

fn from_request(user_request: &SaveUserRequest) -> UserData {
    UserData::new(
        &user_request.first_name,
        &user_request.last_name,
        &user_request.email,
    )
}
