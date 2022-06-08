use std::time::Duration;

use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use conduit_core::config::ConduitConfig;
use conduit_core::errors::{ConduitError, ConduitResult};
use conduit_core::services::token_service::TokenService;

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    user_id: i64,
    exp: usize,
}

pub struct JwtService {
    config: ConduitConfig,
}

impl JwtService {
    pub fn new(config: ConduitConfig) -> Self {
        Self { config }
    }
}

impl TokenService for JwtService {
    fn new_token(&self, user_id: i64, email: &str) -> ConduitResult<String> {
        let expires_in = Duration::from_secs(3600).as_secs();

        let claims = Claims {
            sub: String::from(email),
            exp: expires_in as usize,
            user_id,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.config.token_secret.as_bytes()),
        )
        .map_err(|err| ConduitError::InternalServerErrorWithContext(err.to_string()))?;

        Ok(token)
    }
}
