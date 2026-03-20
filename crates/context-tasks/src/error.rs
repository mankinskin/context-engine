use thiserror::Error;

#[derive(Debug, Error)]
pub enum SchemaValidationError {
    #[error("required field missing: {0}")]
    MissingRequiredField(String),
    #[error("unknown state transition: {from} -> {to}")]
    InvalidTransition { from: String, to: String },
    #[error("edge kind not allowed: {0}")]
    InvalidEdgeKind(String),
}

#[derive(Debug, Error)]
pub enum QueryParseError {
    #[error("invalid query expression: {0}")]
    InvalidExpression(String),
}

#[derive(Debug, Error)]
pub enum StorageSchemaError {
    #[error(
        "schema version mismatch: found '{found}', expected '{expected}'. Action: run 'ticket scan --reindex' after migration or apply schema upgrade before writing"
    )]
    VersionMismatch { found: String, expected: String },
}
