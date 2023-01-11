use actix_web::{dev::ServiceRequest, error::ErrorUnauthorized, Error};
use actix_web_grants::permissions::AttachPermissions;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use log::error;

use super::security_jwt::decode_jwt;

const IGNORED_PATH: &str = "/security";

pub async fn jwt_credentials_extractor(
    request: ServiceRequest,
    credentials: Option<BearerAuth>,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    // used for generating token, so always allowed
    if request.path().contains(IGNORED_PATH) {
        return Ok(request);
    }

    if credentials.is_none() {
        return Err((ErrorUnauthorized("Missing token"), request));
    }

    let credentials = credentials.unwrap();
    let result = decode_jwt(credentials.token());

    match result {
        Ok(user_details) => {
            request.attach(user_details.permissions);
            Ok(request)
        }
        Err(error) => {
            error!("Error occurred while validating token: {:?}", error);

            Err((ErrorUnauthorized("Invalid token"), request))
        }
    }
}
