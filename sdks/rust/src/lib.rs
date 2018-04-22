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
    let _ = cli.ready(&req).map(Box::new).map_err(SdkError::GRPC)?;
    Ok(Box::new(cli))
}

impl Sdk for SdkClient {

    fn ready(&self) -> Result<()> {
        let req = sdk::Empty::default_instance();
        self.ready(req).map(|_| ()).map_err(SdkError::GRPC)
    }

    fn shutdown(&self) -> Result<()> {
        let req = sdk::Empty::default_instance();
        self.shutdown(req).map(|_| ()).map_err(SdkError::GRPC)
    }

    fn health(&self) -> Result<()> {
        self.health().map(|_| ()).map_err(SdkError::GRPC)
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
