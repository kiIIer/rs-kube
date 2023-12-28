use anyhow::Result;
use server::server::run_server;

#[tokio::main]
async fn main() -> Result<()> {
    run_server().await
}
