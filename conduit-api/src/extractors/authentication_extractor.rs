use async_trait::async_trait;
use axum::extract::{FromRequest, RequestParts};
use axum::Extension;
use http::header::AUTHORIZATION;
use tracing::error;

use conduit_core::errors::ConduitError;
use conduit_core::services::token_service::DynTokenService;

/// Extracts the JWT from the Authorization token header.
pub struct AuthenticationExtractor(pub i64);

#[async_trait]
impl<B> FromRequest<B> for AuthenticationExtractor
where
    B: Send + Sync,
{
    type Rejection = ConduitError;

    async fn from_request(request: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Extension(token_service): Extension<DynTokenService> = Extension::from_request(request)
            .await
            .map_err(|err| ConduitError::InternalServerErrorWithContext(err.to_string()))?;

        if let Some(authorization_header) = request.headers().get(AUTHORIZATION) {
            let header_value = authorization_header
                .to_str()
                .map_err(|_| ConduitError::Unauthorized)?;

            if !header_value.contains("Token") {
                error!("request does not contain valid 'Token' prefix for authorization");
                return Err(ConduitError::Unauthorized);
            }

            let tokenized_value: Vec<_> = header_value.split(' ').collect();

            if tokenized_value.len() != 2 || tokenized_value.get(1).is_none() {
                error!("request does not contain a valid token");
                return Err(ConduitError::Unauthorized);
            }

            let token_value = tokenized_value.into_iter().nth(1).unwrap();
            let user_id = token_service
                .get_user_id_from_token(String::from(token_value))
                .map_err(|_| ConduitError::Unauthorized)?;

            Ok(AuthenticationExtractor(user_id))
        } else {
            Err(ConduitError::Unauthorized)
        }
    }
}
