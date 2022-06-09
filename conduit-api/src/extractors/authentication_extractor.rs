use async_trait::async_trait;
use axum::extract::{FromRequest, RequestParts};
use http::header::AUTHORIZATION;

use conduit_core::errors::ConduitError;

/// Extracts the JWT from the Authorization token header.
pub struct AuthenticationExtractor(pub String);

#[async_trait]
impl<B> FromRequest<B> for AuthenticationExtractor
where
    B: Send,
{
    type Rejection = ConduitError;

    async fn from_request(request: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        if let Some(authorization_header) = request.headers().get(AUTHORIZATION) {
            let header_value = authorization_header
                .to_str()
                .map_err(|_| ConduitError::Unauthorized)?;

            if !header_value.contains("Token") {
                return Err(ConduitError::Unauthorized);
            }

            let tokenized_value: Vec<_> = header_value.split(' ').collect();
            if tokenized_value.len() != 2 || tokenized_value.get(1).is_none() {
                return Err(ConduitError::Unauthorized);
            }

            let token_value = tokenized_value.into_iter().nth(1).unwrap();

            Ok(AuthenticationExtractor(String::from(token_value)))
        } else {
            Err(ConduitError::Unauthorized)
        }
    }
}
