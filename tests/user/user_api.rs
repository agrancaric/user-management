use actix_test::TestRequest;
use actix_web::{http::StatusCode, test};
use assert_json_diff::{assert_json_eq, assert_json_include};
use serde_json::{json, Value};
use test_case::test_case;

use crate::user::user_test_util::{
    create_user_json, init_application_and_execute_request, save_default_user,
};

#[actix_web::test]
async fn should_find_all_users() {
    // given
    let request = TestRequest::get().uri("/users?offset=0&limit=10");

    // when
    let response = init_application_and_execute_request(request, "UM_USER_FIND_ALL").await;

    // then
    assert!(response.status().is_success());
}

#[actix_web::test]
async fn should_find_user_by_id() {
    // given
    let user = save_default_user().await;
    let request = TestRequest::get().uri(&format!("/users/{}", user.id));

    // when
    let response = init_application_and_execute_request(request, "UM_USER_FIND_BY_ID").await;

    // then
    assert!(response.status().is_success());

    // and when
    let result: Value = test::read_body_json(response).await;

    // then
    assert_json_eq!(result, json!(user));
}

#[actix_web::test]
async fn should_save_user() {
    // given
    let user = json!({"first_name": "first", "last_name": "last", "email": "email@test.com"});
    let request = TestRequest::post().uri("/users").set_json(&user);

    // when
    let response = init_application_and_execute_request(request, "UM_USER_SAVE").await;

    // then
    assert!(response.response().status().is_success());

    // and when
    let result: Value = test::read_body_json(response).await;

    // then
    assert_json_include!(actual: result, expected: user);
}

#[actix_web::test]
async fn should_report_error_if_save_user_data_is_invalid() {
    // given
    let user = json!({"first_name": "first", "email": "email@test.com"});
    let request = TestRequest::post().uri("/users").set_json(&user);

    // when
    let response = init_application_and_execute_request(request, "UM_USER_SAVE").await;

    // then
    assert!(response.response().status().is_client_error());
}

#[actix_web::test]
async fn should_update_user() {
    // given
    let user = save_default_user().await;
    let updated_user = json!({ "id": user.id, "first_name": "updated_first", "last_name": "updated_last", "email": "updated@test.com"});
    let request = TestRequest::put()
        .uri(&format!("/users/{}", user.id))
        .set_json(&updated_user);

    // when
    let response = init_application_and_execute_request(request, "UM_USER_UPDATE").await;

    // then
    assert!(response.response().status().is_success());

    // and when
    let result: Value = test::read_body_json(response).await;

    // then
    assert_json_eq!(result, updated_user);
}

#[actix_web::test]
async fn should_report_error_if_user_doesnt_exist() {
    // given
    let updated_user = json!({"first_name": "updated_first", "last_name": "updated_last", "email": "updated@test.com"});
    let request = TestRequest::put()
        .uri("/users/11111111")
        .set_json(&updated_user);

    // when
    let response = init_application_and_execute_request(request, "UM_USER_UPDATE").await;

    // then
    assert!(response.response().status().is_server_error());

    // and when
    let result: Value = test::read_body_json(response).await;

    // then
    assert_json_include!(
        actual: result,
        expected: json!({"error": "Record not found"})
    );
}

#[actix_web::test]
async fn should_delete_user() {
    // given
    let user = save_default_user().await;
    let request = TestRequest::delete().uri(&format!("/users/{}", user.id));

    // when
    let response = init_application_and_execute_request(request, "UM_USER_DELETE").await;

    // then
    assert!(response.response().status().is_success());
}

#[test_case(TestRequest::get().uri("/users?offset=0&limit=10"); "when finding all users")]
#[test_case(TestRequest::get().uri("/users/1"); "when finding user by id")]
#[test_case(TestRequest::post().uri("/users").set_json(create_user_json()); "when saving an user")]
#[test_case(TestRequest::put().uri("/users/1").set_json(create_user_json()); "when updating an user")]
#[test_case(TestRequest::delete().uri("/users/1"); "when deleting a user")]
#[actix_web::test]
async fn should_return_forbidden_status_for_invalid_permissions(request: TestRequest) {
    // when
    let response = init_application_and_execute_request(request, "INVALID").await;

    println!("responsse: {:?}", response.response());

    // then
    assert_eq!(response.response().status(), StatusCode::FORBIDDEN);
}
