pub mod server;
pub mod utils;

pub mod messenger_service {
    tonic::include_proto!("messenger");
}
