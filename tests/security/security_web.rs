use actix_test::TestRequest;

use user_management::security::security_web::jwt_credentials_extractor;

// since BearerAuth has private fields other tests cases are not covered here
#[actix_web::test]
async fn should_skip_token_validation_for_security_endpoint() {
    // given
    let request = TestRequest::post().uri("/security").to_srv_request();

    // when
    let result = jwt_credentials_extractor(request, Option::None).await;

    // then
    assert!(result.is_ok());
}

#[actix_web::test]
async fn should_return_error_on_missing_token() {
    // given
    let request = TestRequest::post().uri("/secured").to_srv_request();

    // when
    let result = jwt_credentials_extractor(request, Option::None).await;

    // then
    assert!(result.is_err());
    assert!(result.unwrap_err().0.to_string().contains("Missing token"));
}
