use serde::{Deserialize, Serialize};

pub const COMMAND_SCHEMA_VERSION: &str = "0";

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

impl TicketCommand {
    pub const fn names() -> &'static [&'static str] {
        &[
            "create",
            "get",
            "update",
            "list",
            "delete",
            "scan",
            "claim",
            "unclaim",
            "search",
            "query",
            "history",
            "diff",
            "revert",
            "finalize_merge",
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CommandSchemaExport {
    pub version: String,
    pub command_namespace: String,
    pub commands: Vec<String>,
}

pub fn export_command_schema() -> CommandSchemaExport {
    CommandSchemaExport {
        version: COMMAND_SCHEMA_VERSION.to_string(),
        command_namespace: "ticket".to_string(),
        commands: TicketCommand::names()
            .iter()
            .map(|s| (*s).to_string())
            .collect(),
    }
}

pub fn export_command_schema_json() -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(&export_command_schema())
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
