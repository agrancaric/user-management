use std::env;

use user_management::properties::UserManagementProperties;

#[test]
pub fn should_load_properties_from_environment() {
    // given
    let server_address = "0.0.0.0";
    let server_port = "8100";
    let database_url = "url";
    let database_pool_size = "10";
    let jwt_secret = "secret";

    env::set_var("SERVER_ADDRESS", server_address);
    env::set_var("SERVER_PORT", server_port);
    env::set_var("DATABASE_URL", database_url);
    env::set_var("DATABASE_POOL_SIZE", database_pool_size);
    env::set_var("JWT_SECRET", jwt_secret);

    // when
    let result = envy::from_env::<UserManagementProperties>();

    // then
    assert!(result.is_ok());

    let result = result.unwrap();

    assert_eq!(result.server_address, server_address);
    assert_eq!(result.server_port.to_string(), server_port);
    assert_eq!(result.database_url, database_url);
    assert_eq!(result.database_pool_size.to_string(), database_pool_size);
    assert_eq!(result.jwt_secret, jwt_secret)
}
