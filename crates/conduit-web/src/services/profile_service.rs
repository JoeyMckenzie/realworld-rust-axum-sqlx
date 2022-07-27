use conduit_domain::profiles::{responses::ProfileResponse, ProfileDto};
use lazy_static::lazy_static;
use log::{error, info};

use crate::utilities::{
    errors::{ConduitWebError, ConduitWebResult},
    http::get,
};

lazy_static! {
    static ref PROFILE_ENDPOINT: &'static str = "/profiles";
}

pub async fn get_profile(username: String) -> ConduitWebResult<ProfileDto> {
    let response = get::<ProfileResponse>(&format!("{}/{}", *PROFILE_ENDPOINT, username)).await;

    if let Ok(profile_response) = response {
        info!("{} profile successfully retrieved", username);
        return Ok(profile_response.profile);
    }

    error!("{} profile was not found", username);

    Err(ConduitWebError::ProfileNotFound)
}
