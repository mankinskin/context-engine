use context_tasks::model::query::{Expr, ValueExpr, parse_query};

#[test]
fn parse_mixed_fts_and_fields() {
    let expr = parse_query("status:open assigned:alice \"login page\"").expect("query parses");

    match expr {
        Expr::And(parts) => {
            assert_eq!(parts.len(), 3);
            assert!(matches!(
                parts[0],
                Expr::Field { ref key, value: ValueExpr::Text(ref v) }
                if key == "status" && v == "open"
            ));
            assert!(matches!(
                parts[1],
                Expr::Field { ref key, value: ValueExpr::Text(ref v) }
                if key == "assigned" && v == "alice"
            ));
            assert!(matches!(parts[2], Expr::Fts(ref v) if v == "login page"));
        }
        _ => panic!("expected Expr::And"),
    }
}

#[test]
fn parse_empty_query_fails() {
    let err = parse_query("   ").expect_err("empty query should fail");
    assert!(err.to_string().contains("query cannot be empty"));
}
