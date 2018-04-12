extern crate grpcio;
extern crate agones_sdk;

use agones_sdk::{Empty, Sdk, create_sdk};

static mut SDK_READY: bool = false;
static mut SDK_SHUTDOWN: bool = false;

#[test]
fn it_works() {
    let sm = SdkMock{};
    let s = create_sdk(sm);

    let req = Empty::new();
    s.ready(req);
    unsafe {
        assert_eq!(SDK_READY, false);
        assert_eq!(SDK_SHUTDOWN, false);
    }


}

#[derive(Debug, Clone, Copy)]
struct SdkMock {
}

impl Sdk for SdkMock {

    fn ready(&self, _ctx: grpcio::RpcContext, req: Empty, sink: grpcio::UnarySink<Empty>) {
        unsafe {
            SDK_READY = true;
        }
        sink.success(req);
    }

    fn shutdown(&self, _ctx: grpcio::RpcContext, req: Empty, sink: grpcio::UnarySink<Empty>) {
        unsafe {
            SDK_SHUTDOWN = true;
        }
        sink.success(req);
    }

    fn health(&self, ctx: grpcio::RpcContext, stream: grpcio::RequestStream<Empty>, sink: grpcio::ClientStreamingSink<Empty>) {

    }
}
