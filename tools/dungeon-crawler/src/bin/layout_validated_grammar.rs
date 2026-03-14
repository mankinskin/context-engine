use context_api::ascii_graph::{
    AsciiRule,
    render_layered_ascii,
};

fn grammar() -> Vec<AsciiRule<'static>> {
    vec![
        AsciiRule {
            parent: "ab",
            patterns: &[&["a", "b"]],
        },
        AsciiRule {
            parent: "aba",
            patterns: &[&["ab", "a"]],
        },
        AsciiRule {
            parent: "abab",
            patterns: &[&["ab", "ab"], &["aba", "b"]],
        },
        AsciiRule {
            parent: "ababa",
            patterns: &[&["ab", "aba"], &["abab", "a"]],
        },
        AsciiRule {
            parent: "ababab",
            patterns: &[&["ab", "abab"], &["ababa", "b"]],
        },
        AsciiRule {
            parent: "caba",
            patterns: &[&["c", "aba"]],
        },
        AsciiRule {
            parent: "abc",
            patterns: &[&["ab", "c"]],
        },
        AsciiRule {
            parent: "abcaba",
            patterns: &[&["ab", "caba"], &["abc", "aba"]],
        },
        AsciiRule {
            parent: "abcabab",
            patterns: &[&["abc", "abab"], &["abcaba", "b"]],
        },
        AsciiRule {
            parent: "abcababa",
            patterns: &[&["abc", "ababa"], &["abcabab", "a"]],
        },
        AsciiRule {
            parent: "abcababab",
            patterns: &[&["abc", "ababab"], &["abcababa", "b"]],
        },
        AsciiRule {
            parent: "ababcaba",
            patterns: &[&["ab", "abcaba"], &["abab", "caba"]],
        },
        AsciiRule {
            parent: "abababcaba",
            patterns: &[&["ab", "ababcaba"], &["ababab", "caba"]],
        },
        AsciiRule {
            parent: "abcabababcaba",
            patterns: &[&["abc", "abababcaba"], &["abcababab", "caba"]],
        },
    ]
}

fn main() {
    let rules = grammar();
    match render_layered_ascii(&rules) {
        Ok(ascii) => println!("{}", ascii),
        Err(err) => {
            eprintln!("ERROR: {err}");
            std::process::exit(1);
        },
    }
}
