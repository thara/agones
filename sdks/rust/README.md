# Rust Game Server Client SDK

This is the Rust version of the Agones Game Server Client SDK. 
Check the [Client SDK Documentation](../) for more details on each of the SDK functions and how to run the SDK locally.

> protoc --rust_out sdks/rust/src --grpc_out=sdks/rust/src --plugin=protoc-gen-grpc=`which grpc_rust_plugin` sdk.proto

```toml
[dependencies]
agnoes_sdk = { path = "../agnoes/sdks/rust" }
```

```rust
extern crate grpcio;
extern crate agones_sdk;

use grpcio::{ChannelBuilder, EnvBuilder};
use agones_sdk::{Empty, SdkClient};

use std::sync::Arc;
```

```rust
let env = Arc::new(EnvBuilder::new().build());
let ch = ChannelBuilder::new(env).connect("localhost:50051");
let client = SdkClient::new(ch);
```

```rust
let req = Empty::new();
let result = client.ready(&req).expect('ready');
```

```rust
client.health();
```

```shutdown
let req = Empty::new();
let result = client.shutdown(&req).expect('shutdown');
```
