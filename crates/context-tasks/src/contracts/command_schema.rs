use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "command", rename_all = "snake_case")]
pub enum TicketCommand {
    Create,
    Get,
    Update,
    List,
    Delete,
    Scan,
    Claim,
    Unclaim,
    Search,
    Query,
    History,
    Diff,
    Revert,
    FinalizeMerge,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommandEnvelope {
    pub request_id: String,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ErrorEnvelope {
    pub code: String,
    pub message: String,
}
