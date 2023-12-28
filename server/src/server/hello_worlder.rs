use anyhow::Result;
use tonic::{Request, Response, Status};

use crate::messenger_service::messenger_server::Messenger;
use crate::messenger_service::{HelloRequest, HelloResponse};

#[derive(Default)]
pub struct MyMessenger {}

#[tonic::async_trait]
impl Messenger for MyMessenger {
    async fn hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        println!("Got a request");

        let reply = HelloResponse {};

        Ok(Response::new(reply))
    }
}
