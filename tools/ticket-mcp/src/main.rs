mod server;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("ticket_mcp=info".parse().unwrap()),
        )
        .with_writer(std::io::stderr)
        .init();

    let base_url = std::env::var("TICKET_API_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:4000".to_string());

    eprintln!("ticket-mcp starting (ticket API: {base_url})");

    if let Err(err) = server::run_mcp_server(base_url).await {
        eprintln!("Fatal error: {err}");
        std::process::exit(1);
    }
}
