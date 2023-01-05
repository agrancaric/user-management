use actix_test::TestRequest;
use actix_web::{dev::ServiceResponse, http::header::ContentType, test, App};
use diesel_async::RunQueryDsl;
use user_management::{
    schema::user,
    user::{
        user_api,
        user_models::{User, UserData},
        user_service::UserService,
    },
};

use crate::common_test::test_util::USER_MANAGEMENT_TEST_ENVIRONMENT_CONTEXT;

pub async fn init_application_and_execute_request(request: TestRequest) -> ServiceResponse {
    let application = test::init_service(App::new().service(user_api::init(
        UserService::new(USER_MANAGEMENT_TEST_ENVIRONMENT_CONTEXT.pool.clone()).clone(),
    )))
    .await;

    let request = request.insert_header(ContentType::json()).to_request();

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

    let user = UserData {
        first_name,
        last_name,
        email,
    };

    diesel::insert_into(user::table)
        .values(&user)
        .get_result(&mut *connection)
        .await
        .unwrap()
}
