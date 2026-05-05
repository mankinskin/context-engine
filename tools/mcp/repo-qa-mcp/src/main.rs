use std::path::PathBuf;

use repo_qa_mcp::server;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("repo_qa_mcp=info".parse().unwrap()),
        )
        .with_writer(std::io::stderr)
        .init();

    let base_dir = std::env::current_dir().unwrap_or_else(|_| {
        eprintln!("Warning: could not determine current directory, using '.'");
        PathBuf::from(".")
    });

    eprintln!("repo-qa-mcp starting (base_dir: {})", base_dir.display());

    if let Err(err) = server::run_mcp_server(base_dir).await {
        eprintln!("Fatal error: {err}");
        std::process::exit(1);
    }
}