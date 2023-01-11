use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserDetails {
    pub username: String,
    pub permissions: Vec<String>,
}
