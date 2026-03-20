use serde::{Deserialize, Serialize};

use crate::error::QueryParseError;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ValueExpr {
    Text(String),
    Range { start: String, end: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Expr {
    And(Vec<Expr>),
    Fts(String),
    Field { key: String, value: ValueExpr },
}

pub fn parse_query(input: &str) -> Result<Expr, QueryParseError> {
    let tokens = tokenize(input);
    if tokens.is_empty() {
        return Err(QueryParseError::InvalidExpression("query cannot be empty".to_string()));
    }

    let mut exprs = Vec::with_capacity(tokens.len());
    for token in tokens {
        if let Some((key, raw_value)) = token.split_once(':') {
            if key.is_empty() || raw_value.is_empty() {
                return Err(QueryParseError::InvalidExpression(format!(
                    "invalid field predicate: {token}"
                )));
            }

            let value = if raw_value.starts_with('[') && raw_value.ends_with(']') && raw_value.contains(" TO ") {
                let inner = &raw_value[1..raw_value.len() - 1];
                let (start, end) = inner.split_once(" TO ").ok_or_else(|| {
                    QueryParseError::InvalidExpression(format!("invalid range expression: {token}"))
                })?;
                ValueExpr::Range {
                    start: start.to_string(),
                    end: end.to_string(),
                }
            } else {
                ValueExpr::Text(trim_quotes(raw_value))
            };

            exprs.push(Expr::Field {
                key: key.to_string(),
                value,
            });
        } else {
            exprs.push(Expr::Fts(trim_quotes(&token)));
        }
    }

    Ok(Expr::And(exprs))
}

fn trim_quotes(s: &str) -> String {
    if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
        s[1..s.len() - 1].to_string()
    } else {
        s.to_string()
    }
}

fn tokenize(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;

    for ch in input.chars() {
        match ch {
            '"' => {
                in_quotes = !in_quotes;
                current.push(ch);
            }
            c if c.is_whitespace() && !in_quotes => {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
            }
            _ => current.push(ch),
        }
    }

    if !current.is_empty() {
        tokens.push(current);
    }

    tokens
}
