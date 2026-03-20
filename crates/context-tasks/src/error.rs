use thiserror::Error;
use uuid::Uuid;

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

/// Runtime storage errors covering redb, filesystem, and search index operations.
#[derive(Debug, Error)]
pub enum StorageError {
    #[error("database error: {0}")]
    Database(String),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("serialization error: {0}")]
    Serialization(String),
    #[error("schema version mismatch: {0}")]
    SchemaMismatch(#[from] StorageSchemaError),
    #[error("schema validation: {0}")]
    Validation(#[from] SchemaValidationError),
    #[error("query parse: {0}")]
    QueryParse(#[from] QueryParseError),
    #[error("ticket not found: {0}")]
    NotFound(Uuid),
    #[error("ticket lease conflict: ticket {ticket} held by {holder}")]
    LeaseConflict { ticket: Uuid, holder: String },
    #[error("dependency cycle detected between tickets")]
    DependencyCycle,
    #[error("search index error: {0}")]
    SearchIndex(String),
    #[error("parse diagnostic: {path}: {reason}", path = path.display())]
    ParseError {
        path: std::path::PathBuf,
        reason: String,
    },
}

// Blanket redb error conversions — all redb error types stringify nicely.
impl From<redb::DatabaseError> for StorageError {
    fn from(e: redb::DatabaseError) -> Self {
        StorageError::Database(e.to_string())
    }
}
impl From<redb::TransactionError> for StorageError {
    fn from(e: redb::TransactionError) -> Self {
        StorageError::Database(e.to_string())
    }
}
impl From<redb::TableError> for StorageError {
    fn from(e: redb::TableError) -> Self {
        StorageError::Database(e.to_string())
    }
}
impl From<redb::StorageError> for StorageError {
    fn from(e: redb::StorageError) -> Self {
        StorageError::Database(e.to_string())
    }
}
impl From<redb::CommitError> for StorageError {
    fn from(e: redb::CommitError) -> Self {
        StorageError::Database(e.to_string())
    }
}
