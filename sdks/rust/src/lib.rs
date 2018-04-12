extern crate grpcio;
extern crate grpcio_proto;
extern crate protobuf;
extern crate futures;

mod sdk;
mod sdk_grpc;

pub use sdk::*;
pub use sdk_grpc::*;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
