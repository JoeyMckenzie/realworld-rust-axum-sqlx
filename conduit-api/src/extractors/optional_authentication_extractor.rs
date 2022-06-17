use async_trait::async_trait;
use axum::extract::{FromRequest, RequestParts};
use axum::Extension;
use http::header::AUTHORIZATION;
use tracing::warn;

use conduit_core::errors::ConduitError;
use conduit_core::utils::token_service::DynTokenService;

/// Extracts the JWT from the Authorization token header, optional and will not return errors if none is found.
pub struct OptionalAuthentication(pub Option<i64>);

#[async_trait]
impl<B> FromRequest<B> for OptionalAuthentication
where
    B: Send + Sync,
{
    type Rejection = ConduitError;

    async fn from_request(request: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let optional_token_response = Ok(OptionalAuthentication(None));

        let Extension(token_service): Extension<DynTokenService> = Extension::from_request(request)
            .await
            .map_err(|err| ConduitError::InternalServerErrorWithContext(err.to_string()))?;

        if let Some(authorization_header) = request.headers().get(AUTHORIZATION) {
            if let Ok(header_value) = authorization_header.to_str() {
                if !header_value.contains("Token") {
                    warn!("request does not contain valid 'Token' prefix for authorization");
                    return optional_token_response;
                }

                let tokenized_value: Vec<_> = header_value.split(' ').collect();

                if tokenized_value.len() != 2 || tokenized_value.get(1).is_none() {
                    warn!("request does not contain a valid token");
                    return optional_token_response;
                }

                let token_value = tokenized_value.into_iter().nth(1).unwrap();

                if let Ok(user_id) = token_service.get_user_id_from_token(String::from(token_value))
                {
                    return Ok(OptionalAuthentication(Some(user_id)));
                }
            }
        }

        optional_token_response
    }
}
