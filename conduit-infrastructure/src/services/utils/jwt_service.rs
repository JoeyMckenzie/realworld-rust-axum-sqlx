use std::ops::Add;
use std::time::Duration;

use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

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
        let from_now = Duration::from_secs(3600);
        let exp = OffsetDateTime::now_utc().add(from_now).unix_timestamp();

        let claims = Claims {
            sub: String::from(email),
            exp: exp as usize,
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

    fn get_user_id_from_token(&self, token: String) -> ConduitResult<i64> {
        let decoded_token = decode::<Claims>(
            token.as_str(),
            &DecodingKey::from_secret(self.config.token_secret.as_bytes()),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|err| ConduitError::InternalServerErrorWithContext(err.to_string()))?;

        Ok(decoded_token.claims.user_id)
    }
}
