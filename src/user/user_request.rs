use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct SaveUserRequest {
    #[validate(length(min = 1, max = 150))]
    pub first_name: String,
    #[validate(length(min = 1, max = 150))]
    pub last_name: String,
    #[validate(email)]
    pub email: String,
}
