extern crate grpcio;
extern crate grpcio_proto;
extern crate protobuf;
extern crate futures;

mod error;
mod sdk;
mod sdk_grpc;

use error::SdkError;
use std::result;
use std::sync::Arc;
use std::time::Duration;
use grpcio::{ChannelBuilder, EnvBuilder};

pub type Result<T> = result::Result<T, error::SdkError>;
pub type SdkClient = sdk_grpc::SdkClient;

pub trait Sdk {
    fn ready(&self) -> Result<()>;
    fn shutdown(&self) -> Result<()>;
    fn health(&self) -> Result<()>;
}

const PORT: i32 = 59357;

pub fn new_sdk() -> Result<Box<Sdk>> {
    let addr = format!("localhost:{}", PORT);
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).keepalive_timeout(Duration::new(30, 0)).connect(&addr);
    let cli = sdk_grpc::SdkClient::new(ch);
    let req = sdk::Empty::new();
    let _ = cli.ready(&req).map(Box::new).map_err(SdkError::Grpc)?;
    Ok(Box::new(cli))
}

impl Sdk for SdkClient {

    fn ready(&self) -> Result<()> {
        let req = sdk::Empty::default_instance();
        self.ready(req).map(|_| ()).map_err(SdkError::Grpc)
    }

    fn shutdown(&self) -> Result<()> {
        let req = sdk::Empty::default_instance();
        self.shutdown(req).map(|_| ()).map_err(SdkError::Grpc)
    }

    fn health(&self) -> Result<()> {
        self.health().map(|_| ()).map_err(SdkError::Grpc)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        match ::new_sdk() {
            Ok(sdk) => {
                let _ = sdk.ready();
                let _ = sdk.health();
                let _ = sdk.shutdown();
            },
            Err(_) => {
            }
        }
    }
}
