use crate::utils::{db_connection_manager, rabbit_channel_manager};
use anyhow::Result;
use std::time::Duration;
use tokio::time::sleep;

pub async fn run_worker() -> Result<()> {
    let db_connection_manager = db_connection_manager::get_connection_manager();
    let rabbit_channel_manager = rabbit_channel_manager::get_channel_manager();

    let conneciton = db_connection_manager.get_connection()?;
    let channel = rabbit_channel_manager.get_channel().await?;
    println!("Database and RabbitMQ are operational.");

    println!("Waiting for one day...");
    sleep(Duration::from_secs(86400)).await;

    println!("One day has passed lol.");

    Ok(())
}
