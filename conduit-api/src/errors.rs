use axum::response::Response;
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use std::{borrow::Cow, collections::HashMap, fmt::Debug};
use thiserror::Error;

pub type ConduitEndpointResult<T> = Result<T, ConduitError>;

pub type ConduitErrorMap = HashMap<Cow<'static, str>, Vec<Cow<'static, str>>>;

#[derive(Error, Debug)]
pub enum ConduitError {
    #[error("authentication is required to access this resource")]
    Unauthorized,
    #[error("user does not have privilege to access this resource")]
    Forbidden,
    #[error("requested resource was not found")]
    NotFound,
    #[error("{0}")]
    ApplicationStartup(&'static str),
    #[error("{0}")]
    BadRequest(&'static str),
    #[error("unexpected error has occurred")]
    InternalServerError,
    #[error("{0} already exists")]
    ObjectConflict(&'static str),
    #[error("unprocessable request has occurred")]
    UnprocessableEntity { errors: ConduitErrorMap },
}

impl ConduitError {
    pub fn unprocessable_entity<K, V>(errors: impl IntoIterator<Item = (K, V)>) -> Self
    where
        K: Into<Cow<'static, str>>,
        V: Into<Cow<'static, str>>,
    {
        let mut validation_errors = ConduitErrorMap::new();

        for (key, value) in errors {
            validation_errors
                .entry(key.into())
                .or_insert_with(Vec::new)
                .push(value.into())
        }

        Self::UnprocessableEntity {
            errors: validation_errors,
        }
    }

    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::UnprocessableEntity { .. } => StatusCode::UNPROCESSABLE_ENTITY,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for ConduitError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Self::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, "User not found"),
            _ => (StatusCode::NOT_FOUND, "User not found"),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
