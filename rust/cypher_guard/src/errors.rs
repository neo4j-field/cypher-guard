#[derive(Debug, thiserror::Error)]
pub enum CypherGuardError {
    #[error("Invalid query")]
    InvalidQuery,
    #[error("Schema error")]
    SchemaError,
}