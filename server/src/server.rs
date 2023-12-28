use crate::messenger_service;
use crate::utils::{db_connection_manager, rabbit_channel_manager, status_checker};
use anyhow::Result;
use tokio::try_join;
use tonic::transport::Server;

mod hello_worlder;

pub async fn run_server() -> Result<()> {
    check_env_vars();
    let db_connection_manager = db_connection_manager::get_connection_manager();
    let rabbit_channel_manager = rabbit_channel_manager::get_channel_manager();

    let status_checker = status_checker::StatusChecker::new(
        db_connection_manager.clone(),
        rabbit_channel_manager.clone(),
    );

    let hello_worlder = hello_worlder::MyMessenger::default();

    let addr = std::env::var("SERVER_SOCKET_ADDRESS").expect("SERVER_SOCKET_ADDRESS must be set");
    let health_addr = std::env::var("HEALTH_ADDRESS").expect("HEALTH_ADDRESS must be set");

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

fn check_env_vars() {
    dotenv::dotenv().ok();

    let _ = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let _ = std::env::var("RABBIT_HOST").expect("RABBIT_HOST must be set");
    let _ = std::env::var("RABBIT_PORT").expect("RABBIT_PORT must be set");
    let _ = std::env::var("RABBIT_USER").expect("RABBIT_USER must be set");
    let _ = std::env::var("RABBIT_PASSWORD").expect("RABBIT_PASSWORD must be set");
    let _ = std::env::var("SERVER_SOCKET_ADDRESS").expect("SERVER_SOCKET_ADDRESS must be set");
    let _ = std::env::var("AUTH0_CLIENT_ID").expect("AUTH0_CLIENT_ID must be set");
    let _ = std::env::var("AUTH0_CLIENT_SECRET").expect("AUTH0_CLIENT_SECRET must be set");
    let _ = std::env::var("AUTH0_AUDIENCE").expect("AUTH0_AUDIENCE must be set");
    let _ = std::env::var("AUTH0_SERVER_N").expect("AUTH0_SERVER_N must be set");
    let _ = std::env::var("AUTH0_SERVER_E").expect("AUTH0_SERVER_E must be set");
    let _ = std::env::var("HEALTH_ADDRESS").expect("HEALTH_ADDRESS must be set");

    println!("All required environment variables are set.");
}
