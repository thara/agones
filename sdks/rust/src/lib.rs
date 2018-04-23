extern crate grpcio;
extern crate grpcio_proto;
extern crate protobuf;
extern crate futures;

mod sdk;
mod sdk_grpc;

use std::result;
use std::sync::Arc;
use std::time::Duration;
use grpcio::{ChannelBuilder, EnvBuilder};

pub trait Sdk {
    type Error: std::error::Error;
    fn ready(&self) -> result::Result<(), Self::Error>;
    fn shutdown(&self) -> result::Result<(), Self::Error>;
    fn health(&self) -> result::Result<(), Self::Error>;
}

const PORT: i32 = 59357;

pub fn new_sdk() -> result::Result<Box<Sdk<Error=grpcio::Error>>, grpcio::Error> {
    let addr = format!("localhost:{}", PORT);
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).keepalive_timeout(Duration::new(30, 0)).connect(&addr);
    let cli = sdk_grpc::SdkClient::new(ch);
    let req = sdk::Empty::new();
    let _ = cli.ready(&req).map(Box::new)?;
    Ok(Box::new(cli))
}

impl Sdk for sdk_grpc::SdkClient {
    type Error = grpcio::Error;

    fn ready(&self) -> result::Result<(), Self::Error> {
        let req = sdk::Empty::default_instance();
        self.ready(req).map(|_| ())
    }

    fn shutdown(&self) -> result::Result<(), Self::Error> {
        let req = sdk::Empty::default_instance();
        self.shutdown(req).map(|_| ())
    }

    fn health(&self) -> result::Result<(), Self::Error> {
        self.health().map(|_| ())
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
            Err(_err) => {
            }
        }
    }
}
