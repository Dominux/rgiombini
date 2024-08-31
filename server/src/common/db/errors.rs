use crate::errors::RgiombiniError;

#[inline]
pub fn map_not_found(e: sqlx::Error, entity_name: &str) -> RgiombiniError {
    match e {
        sqlx::Error::RowNotFound => RgiombiniError::DoesNotExist(format!("such {entity_name}")),
        _ => {
            tracing::error!("{e}");
            RgiombiniError::Unknown
        }
    }
}
