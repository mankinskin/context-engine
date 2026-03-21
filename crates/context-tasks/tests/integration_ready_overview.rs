mod common;

use common::{Sandbox, create_ticket};

#[test]
fn ready_overview_writes_markdown_with_ready_tickets() {
    let s = Sandbox::new();

    let blocked = create_ticket(&s, "Blocked ticket");
    let done_dependency = create_ticket(&s, "Done dependency");
    let ready = create_ticket(&s, "Ready ticket");

    s.ticket_json(&["update", "--id", &done_dependency, "--to-state", "in-progress"]);
    s.ticket_json(&["update", "--id", &done_dependency, "--to-state", "review"]);
    s.ticket_json(&["update", "--id", &done_dependency, "--to-state", "validating"]);
    s.ticket_json(&["update", "--id", &done_dependency, "--to-state", "validated"]);
    s.ticket_json(&["update", "--id", &done_dependency, "--to-state", "release-candidate"]);
    s.ticket_json(&["update", "--id", &done_dependency, "--to-state", "released"]);
    s.ticket_json(&["update", "--id", &done_dependency, "--to-state", "monitoring"]);
    s.ticket_json(&["update", "--id", &done_dependency, "--to-state", "done"]);

    s.ticket_json(&[
        "link",
        "--from",
        &blocked,
        "--to",
        &ready,
        "--kind",
        "depends_on",
    ]);

    let output_path = s.index_root.join("ready-overview.md");
    let output_path_str = output_path.to_string_lossy().to_string();

    let result = s.ticket_json(&[
        "ready-overview",
        "--output",
        &output_path_str,
        "--scope",
        "integration test scope",
    ]);

    assert_eq!(result["status"], "ok");
    assert_eq!(result["ready_count"], 1);

    let markdown = std::fs::read_to_string(&output_path).expect("overview markdown should exist");
    assert!(markdown.contains("# Ready Tickets Overview"));
    assert!(markdown.contains("Scope: integration test scope"));
    assert!(markdown.contains(&ready));
    assert!(markdown.contains("Ready ticket"));
}
