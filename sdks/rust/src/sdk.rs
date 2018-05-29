use std::sync::Arc;
use std::time::Duration;
use grpcio::{ChannelBuilder, EnvBuilder};

use errors::*;
use grpc::sdk as sdk;
use grpc::sdk_grpc as sdk_grpc;

const PORT: i32 = 59357;

/// SDK is an instance of the Agones SDK
pub struct Sdk {
    client : Arc<sdk_grpc::SdkClient>,
}

impl Sdk {

    /// Starts a new SDK instance, and connects to localhost on port 59357.
    /// Blocks until connection and handshake are made.
    /// Times out after 30 seconds.
    pub fn new() -> Result<Sdk> {
        let addr = format!("localhost:{}", PORT);
        let env = Arc::new(EnvBuilder::new().build());
        let ch = ChannelBuilder::new(env).keepalive_timeout(Duration::new(30, 0)).connect(&addr);
        let cli = sdk_grpc::SdkClient::new(ch);
        let req = sdk::Empty::new();
        let _ = cli.ready(&req).map(Box::new)?;
        Ok(Sdk{client: Arc::new(cli)})
    }

    /// Marks the Game Server as ready to receive connections
    pub fn ready(&self) -> Result<()> {
        let req = sdk::Empty::default_instance();
        let res = self.client.ready(req).map(|_| ())?;
        Ok(res)
    }

    /// Marks the Game Server as ready to shutdown
    pub fn shutdown(&self) -> Result<()> {
        let req = sdk::Empty::default_instance();
        let res = self.client.shutdown(req).map(|_| ())?;
        Ok(res)
    }

    /// Sends a ping to the health check to indicate that this server is healthy
    pub fn health(&self) -> Result<()> {
        let res = self.client.health().map(|_| ())?;
        Ok(res)
    }
}

impl Clone for Sdk {
    fn clone(&self) -> Self {
        Self {
            client: Arc::clone(&self.client),
        }
    }
}
