use axum::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RgiombiniError {
    #[error("environment variable `{0}` is not set")]
    EnvConfigLoadingError(String),
    #[error("environment variable `{0}` cannot be parsed")]
    EnvVarParsingError(String),

    #[error("{0} already exists")]
    AlreadyExists(String),
    #[error("{0} does not exist")]
    DoesNotExist(String),
    #[error("not authenticated")]
    NotAuthenticated,
    #[error("unknown error")]
    Unknown,
}

impl From<RgiombiniError> for (StatusCode, String) {
    fn from(e: RgiombiniError) -> Self {
        match &e {
            RgiombiniError::AlreadyExists(_) => (StatusCode::CONFLICT, e.to_string()),
            RgiombiniError::NotAuthenticated => (StatusCode::UNAUTHORIZED, e.to_string()),
            RgiombiniError::DoesNotExist(_) => (StatusCode::NOT_FOUND, e.to_string()),
            _ => {
                tracing::error!("{e}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Something went wrong".to_owned(),
                )
            }
        }
    }
}

pub type RgiombiniResult<T> = Result<T, RgiombiniError>;
