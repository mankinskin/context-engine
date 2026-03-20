use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::ticket::TicketId;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HistoryEntry {
    pub ticket_id: TicketId,
    pub commit_sha: String,
    pub actor: String,
    pub at: DateTime<Utc>,
}
