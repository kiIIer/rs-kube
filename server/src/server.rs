use crate::messenger_service;
use crate::messenger_service::messenger_server::{Messenger, MessengerServer};
use crate::server::hello_worlder::MyMessenger;
use crate::utils::db_connection_manager::DBConnectionManager;
use crate::utils::rabbit_channel_manager::RabbitChannelManager;
use crate::utils::{db_connection_manager, rabbit_channel_manager};
use anyhow::Result;
use std::sync::Arc;
use tokio::try_join;
use tonic::transport::Server;

mod hello_worlder;

pub async fn run_server() -> Result<()> {
    check_env_vars();

    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_not_serving::<MessengerServer<MyMessenger>>()
        .await;

    let db_connection_manager = db_connection_manager::get_connection_manager();
    let rabbit_channel_manager = rabbit_channel_manager::get_channel_manager();

    tokio::spawn(go_to_doctor(
        health_reporter,
        db_connection_manager.clone(),
        rabbit_channel_manager.clone(),
    ));

    let hello_worlder = MyMessenger::default();

    let addr = std::env::var("SERVER_SOCKET_ADDRESS").expect("SERVER_SOCKET_ADDRESS must be set");
    let health_addr = std::env::var("HEALTH_ADDRESS").expect("HEALTH_ADDRESS must be set");

    let server_handle = Server::builder()
        .add_service(MessengerServer::new(hello_worlder))
        .serve(addr.parse().unwrap());

    let health_server_handle = Server::builder()
        .add_service(health_service)
        .serve(health_addr.parse().unwrap());

    try_join!(server_handle, health_server_handle)?;
    Ok(())
}

async fn go_to_doctor(
    mut health_Reporter: tonic_health::server::HealthReporter,
    db_connection_manager: Arc<dyn DBConnectionManager>,
    rabbit_channel_manager: Arc<dyn RabbitChannelManager>,
) {
    loop {
        let db_connection = db_connection_manager
            .get_connection()
            .expect("Failed to get connection");
        let rabbit_channel = rabbit_channel_manager
            .get_channel()
            .await
            .expect("Failed to get channel");
        health_Reporter
            .set_serving::<MessengerServer<MyMessenger>>()
            .await;
    }
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

    for (key, value) in std::env::vars() {
        println!("{} = {}", key, value);
    }

    println!("All required environment variables are set.");
}
