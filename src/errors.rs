use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum ConduitError {
    StartupError,
}

impl From<hyper::error::Error> for ConduitError {
    fn from(_: hyper::error::Error) -> Self {
        ConduitError::StartupError
    }
}

impl From<std::net::AddrParseError> for ConduitError {
    fn from(_: std::net::AddrParseError) -> Self {
        ConduitError::StartupError
    }
}

impl From<sqlx::migrate::MigrateError> for ConduitError {
    fn from(_: sqlx::migrate::MigrateError) -> Self {
        ConduitError::StartupError
    }
}

impl Display for ConduitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "an unexpected error has occurred")
    }
}
