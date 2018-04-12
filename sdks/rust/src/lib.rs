extern crate grpcio;
extern crate grpcio_proto;
extern crate protobuf;
extern crate futures;

pub mod sdk_grpc;
pub mod sdk;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
