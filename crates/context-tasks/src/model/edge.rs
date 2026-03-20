use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::ticket::TicketId;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EdgeRecord {
    pub from: TicketId,
    pub to: TicketId,
    pub kind: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EdgeKindRule {
    pub directed: bool,
    pub acyclic_enforced: bool,
}
