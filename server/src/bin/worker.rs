use anyhow::Result;
use server::worker::run_worker;

#[tokio::main]
async fn main() -> Result<()> {
    run_worker().await
}
