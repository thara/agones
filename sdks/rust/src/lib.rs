extern crate grpcio;
extern crate grpcio_proto;
extern crate protobuf;
extern crate futures;

mod sdk;
mod sdk_grpc;

use std::{error, fmt, result};
use std::sync::Arc;
use std::time::Duration;
use grpcio::{ChannelBuilder, EnvBuilder};

#[derive(Debug)]
pub enum SdkError {
    GRPC(grpcio::Error),
}

impl fmt::Display for SdkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SdkError::GRPC(ref err) => write!(f, "gRPC error: {}", err),
        }
    }
}

impl error::Error for SdkError {
    fn description(&self) -> &str {
        match *self {
            SdkError::GRPC(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            SdkError::GRPC(ref err) => Some(err),
        }
    }
}

impl From<grpcio::Error> for SdkError {
    fn from(err: grpcio::Error) -> SdkError {
        SdkError::GRPC(err)
    }
}

pub type Result<T> = result::Result<T, SdkError>;

pub struct Sdk {
    client: sdk_grpc::SdkClient,
}

const PORT: i32 = 59357;

impl Sdk {
    pub fn new() -> Result<Self> {
        let addr = format!("localhost:{}", PORT);
        let env = Arc::new(EnvBuilder::new().build());
        let ch = ChannelBuilder::new(env).keepalive_timeout(Duration::new(30, 0)).connect(&addr);
        let cli = sdk_grpc::SdkClient::new(ch);
        let req = sdk::Empty::new();
        let _ = cli.ready(&req).map_err(SdkError::GRPC)?;
        Ok(Sdk{client : cli})
    }

    pub fn ready(&self) -> Result<()> {
        let req = sdk::Empty::default_instance();
        self.client.ready(req).map(|_| ()).map_err(SdkError::GRPC)
    }

    pub fn shutdown(&self) -> Result<()> {
        let req = sdk::Empty::default_instance();
        self.client.shutdown(req).map(|_| ()).map_err(SdkError::GRPC)
    }

    pub fn health(&self) -> Result<()> {
        self.client.health().map(|_| ()).map_err(SdkError::GRPC)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        match ::Sdk::new() {
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
