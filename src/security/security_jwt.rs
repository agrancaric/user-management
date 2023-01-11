use chrono::{Duration, Utc};
use jsonwebtoken::errors::Error;
use jsonwebtoken::{self, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde_json::json;

use super::security_model::UserDetails;

const SECRET: Lazy<String> =
    Lazy::new(|| std::env::var("JWT_SECRET").expect("JWT_SECRET must be set!"));

pub fn encode_jwt(user_details: &UserDetails) -> Result<String, Error> {
    let secret = SECRET;
    let secret = secret.as_bytes();
    let token_expiry = Utc::now() + Duration::hours(8);
    let data = json!({ "username": user_details.username, "permissions": user_details.permissions, "exp": token_expiry.timestamp() });

    jsonwebtoken::encode(&Header::default(), &data, &EncodingKey::from_secret(secret))
}

pub fn decode_jwt(jwt: &str) -> Result<UserDetails, Error> {
    jsonwebtoken::decode::<UserDetails>(
        &jwt,
        &DecodingKey::from_secret(SECRET.as_ref()),
        &Validation::default(),
    )
    .map(|decoded| decoded.claims)
}
