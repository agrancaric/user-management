use actix_test::TestRequest;
use actix_web::{
    body::{BoxBody, EitherBody},
    dev::ServiceResponse,
    http::header::ContentType,
    test, App,
};
use actix_web_httpauth::middleware::HttpAuthentication;
use diesel_async::RunQueryDsl;
use serde_json::{json, Value};

use user_management::{
    schema::user,
    security::{
        security_jwt::encode_jwt, security_model::UserDetails,
        security_web::jwt_credentials_extractor,
    },
    user::{
        user_api,
        user_model::{User, UserData},
        user_service::UserService,
    },
};

use crate::common_test::test_util::USER_MANAGEMENT_TEST_ENVIRONMENT_CONTEXT;

pub async fn init_application_and_execute_request(
    request: TestRequest,
    permission: &str,
) -> ServiceResponse<EitherBody<BoxBody>> {
    let token = get_token(permission);
    let authentication = HttpAuthentication::with_fn(jwt_credentials_extractor);

    let application = test::init_service(App::new().wrap(authentication).service(user_api::init(
        UserService::new(USER_MANAGEMENT_TEST_ENVIRONMENT_CONTEXT.pool.clone()),
    )))
    .await;

    let request = request
        .insert_header(ContentType::json())
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    test::call_service(&application, request).await
}

pub async fn save_default_user() -> User {
    save_user("first", "last", "email@test.com").await
}

pub async fn save_user(first_name: &str, last_name: &str, email: &str) -> User {
    let mut connection = USER_MANAGEMENT_TEST_ENVIRONMENT_CONTEXT
        .pool
        .get()
        .await
        .unwrap();

    let user = UserData::new(first_name, last_name, email);

    diesel::insert_into(user::table)
        .values(&user)
        .get_result(&mut *connection)
        .await
        .unwrap()
}

pub fn create_user_json() -> Value {
    json!({ "first_name": "first", "last_name": "last", "email": "email@test.com" })
}

fn get_token(permission: &str) -> String {
    let user_details = UserDetails {
        username: "username".to_string(),
        permissions: vec![permission.to_string()],
    };

    encode_jwt(&user_details).unwrap()
}
