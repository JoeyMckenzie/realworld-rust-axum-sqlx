use serde::{Deserialize, Serialize};

use crate::profiles::ProfileDto;

#[derive(Deserialize, Serialize)]
pub struct ProfileResponse {
    pub profile: ProfileDto,
}
