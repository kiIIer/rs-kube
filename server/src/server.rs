use crate::utils::{db_connection_manager, rabbit_channel_manager};
use anyhow::Result;

mod hello_worlder;

async fn run_server() -> Result<()> {
    let db_connection_manager = db_connection_manager::get_connection_manager();
    let rabbit_channel_manager = rabbit_channel_manager::get_channel_manager();

    Ok(())
}
