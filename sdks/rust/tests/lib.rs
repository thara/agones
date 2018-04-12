extern crate grpcio;
extern crate agones_sdk;

use std::sync::Arc;

use grpcio::{ChannelBuilder, EnvBuilder};
use agones_sdk::sdk::Empty;
use agones_sdk::sdk_grpc::SdkClient;

#[test]
fn it_works() {
   let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("localhost:50051");
    let client = SdkClient::new(ch);

    let req = Empty::new();
    client.ready(&req).expect("ready");

    let req = Empty::new();
    client.shutdown(&req).expect("shutdown");

    assert_eq!(4, 4);
}
