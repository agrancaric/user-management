use actix_test::TestRequest;
use actix_web::{
    http::{header::ContentType, StatusCode},
    test, App,
};
use serde_json::json;
use user_management::security::security_api;

use crate::security::test_util::init_test_environment;

#[actix_web::test]
async fn should_generate_token() {
    // given
    init_test_environment();
    let user = json!({
        "username": "username",
        "permissions": ["UM_USER_FIND"],
    });
    let application = test::init_service(App::new().service(security_api::init())).await;

    let request = TestRequest::post()
        .uri("/security")
        .set_json(user)
        .insert_header(ContentType::json())
        .to_request();

    // when
    let response = test::call_service(&application, request).await;

    // then
    assert_eq!(response.status(), StatusCode::OK);

    // and when
    let result: String = test::read_body_json(response).await;

    // then
    assert!(!result.is_empty());
}
