use crate::messenger_service;
use crate::utils::{db_connection_manager, rabbit_channel_manager, status_checker};
use anyhow::Result;
use tokio::try_join;
use tonic::transport::Server;

mod hello_worlder;

async fn run_server() -> Result<()> {
    let db_connection_manager = db_connection_manager::get_connection_manager();
    let rabbit_channel_manager = rabbit_channel_manager::get_channel_manager();

    let status_checker = status_checker::StatusChecker::new(
        db_connection_manager.clone(),
        rabbit_channel_manager.clone(),
    );

    let hello_worlder = hello_worlder::MyMessenger::default();

    let addr = std::env::var("SERVER_ADDR").expect("SERVER_ADDR must be set");
    let health_addr = std::env::var("HEALTH_ADDR").expect("HEALTH_ADDR must be set");

    let server_handle = Server::builder()
        .add_service(messenger_service::messenger_server::MessengerServer::new(
            hello_worlder,
        ))
        .serve(addr.parse().unwrap());

    let health_server_handle = Server::builder()
        .add_service(messenger_service::health_server::HealthServer::new(
            status_checker,
        ))
        .serve(health_addr.parse().unwrap());

    try_join!(server_handle, health_server_handle)?;
    Ok(())
}
