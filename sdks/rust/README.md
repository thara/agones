# Rust Game Server Client SDK

This is the Rust version of the Agones Game Server Client SDK. 
Check the [Client SDK Documentation](../) for more details on each of the SDK functions and how to run the SDK locally.

> protoc --rust_out sdks/rust/src --grpc_out=sdks/rust/src --plugin=protoc-gen-grpc=`which grpc_rust_plugin` sdk.proto


