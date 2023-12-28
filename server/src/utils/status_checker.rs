use crate::messenger_service::health_server::Health;
use crate::messenger_service::{HealthCheckRequest, HealthCheckResponse};
use crate::utils::db_connection_manager::DBConnectionManager;
use crate::utils::rabbit_channel_manager::RabbitChannelManager;
use diesel::r2d2::R2D2Connection;
use std::sync::Arc;
use tonic::{Request, Response, Status};

pub struct StatusChecker {
    db_connection_manager: Arc<dyn DBConnectionManager>,
    rabbit_channel_manager: Arc<dyn RabbitChannelManager>,
}

impl StatusChecker {
    pub fn new(
        db_connection_manager: Arc<dyn DBConnectionManager>,
        rabbit_channel_manager: Arc<dyn RabbitChannelManager>,
    ) -> Self {
        Self {
            db_connection_manager,
            rabbit_channel_manager,
        }
    }
}

#[tonic::async_trait]
impl Health for StatusChecker {
    async fn check(
        &self,
        request: Request<HealthCheckRequest>,
    ) -> Result<Response<HealthCheckResponse>, Status> {
        println!("Got a health check request");
        let db_connection = self
            .db_connection_manager
            .get_connection()
            .expect("Failed to get connection");
        let rabbit_channel = self
            .rabbit_channel_manager
            .get_channel()
            .await
            .expect("Failed to get channel");

        let reply = HealthCheckResponse {};

        Ok(Response::new(reply))
    }
}
