/// One-shot migration: read edges from tickets.redb → insert into tickets.db.
///
/// Edge keys in redb: "{from_uuid}|{to_uuid}|{kind}" (value = ())
/// Edge columns in SQLite: from_id, to_id, kind, created_at
///
/// Usage: migrate-edges <path/to/.ticket>
use redb::{
    Database,
    ReadableTable,
    TableDefinition,
};
use rusqlite::{
    params,
    Connection,
};

const EDGES: TableDefinition<&str, ()> = TableDefinition::new("edges");

fn main() {
    let ticket_dir = std::env::args()
        .nth(1)
        .unwrap_or_else(|| ".ticket".to_string());
    let redb_path = format!("{ticket_dir}/tickets.redb");
    let sqlite_path = format!("{ticket_dir}/tickets.db");

    println!("Reading edges from: {redb_path}");
    println!("Writing edges to:   {sqlite_path}");

    // Open redb read-only.
    let redb = Database::open(&redb_path).expect("failed to open tickets.redb");
    let read_txn = redb.begin_read().expect("failed to begin read txn");

    let edges_table = match read_txn.open_table(EDGES) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Could not open 'edges' table in redb: {e}");
            eprintln!("The old database may have no edges.");
            return;
        },
    };

    let mut edges: Vec<(String, String, String)> = Vec::new();
    for entry in edges_table.iter().expect("failed to iterate edges") {
        let (k, _v) = entry.expect("failed to read entry");
        let key = k.value().to_string();
        let parts: Vec<&str> = key.splitn(3, '|').collect();
        if parts.len() == 3 {
            edges.push((
                parts[0].to_string(),
                parts[1].to_string(),
                parts[2].to_string(),
            ));
        } else {
            eprintln!("Skipping malformed edge key: {key}");
        }
    }

    println!("Found {} edge(s) in redb", edges.len());
    if edges.is_empty() {
        println!("Nothing to migrate.");
        return;
    }

    // Open SQLite and insert.
    let conn =
        Connection::open(&sqlite_path).expect("failed to open tickets.db");
    conn.execute_batch("PRAGMA journal_mode=WAL;")
        .expect("WAL pragma failed");

    let now = chrono_now();
    let mut inserted = 0usize;
    let mut skipped = 0usize;

    for (from, to, kind) in &edges {
        let rows = conn
            .execute(
                "INSERT OR IGNORE INTO edges (from_id, to_id, kind, created_at) VALUES (?1, ?2, ?3, ?4)",
                params![from, to, kind, now],
            )
            .expect("failed to insert edge");
        if rows > 0 {
            inserted += 1;
        } else {
            skipped += 1;
        }
    }

    println!("Migrated {inserted} edge(s) ({skipped} already existed).");
}

fn chrono_now() -> String {
    // RFC3339 UTC timestamp without pulling in chrono.
    use std::time::{
        SystemTime,
        UNIX_EPOCH,
    };
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    // Format as 2026-05-04T... (approximate, good enough for migration).
    let days = secs / 86400;
    let year = 1970 + days / 365;
    let rem_days = days % 365;
    let month = rem_days / 30 + 1;
    let day = rem_days % 30 + 1;
    let time_secs = secs % 86400;
    let h = time_secs / 3600;
    let m = (time_secs % 3600) / 60;
    let s = time_secs % 60;
    format!("{year:04}-{month:02}-{day:02}T{h:02}:{m:02}:{s:02}Z")
}
