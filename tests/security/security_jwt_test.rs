use user_management::security::{
    security_jwt::{decode_jwt, encode_jwt},
    security_model::UserDetails,
};

use crate::security::security_test_util::init_test_environment;

#[test]
fn should_create_and_validate_token() {
    // given
    init_test_environment();

    let user = UserDetails {
        username: "username".to_string(),
        permissions: vec!["UM_USER_FIND".to_string()],
    };

    // when
    let token = encode_jwt(&user);

    // then
    assert!(token.is_ok());

    // and when
    let result = decode_jwt(&token.unwrap());

    // then
    assert!(result.is_ok());

    let result = result.unwrap();

    assert_eq!(result.username, user.username);
    assert_eq!(result.permissions, user.permissions);
}
