use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct UserManagementProperties {
    pub server_address: String,
    pub server_port: u16,
    pub database_url: String,
    pub database_pool_size: usize,
    pub jwt_secret: String,
}
