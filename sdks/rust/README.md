# Rust Game Server Client SDK

This is the Rust version of the Agones Game Server Client SDK. 
Check the [Client SDK Documentation](../) for more details on each of the SDK functions and how to run the SDK locally.

## Installation

Add this crate to `dependencies` section in your Cargo.toml.
Specify a directory where this README.md is located to the `path`.

```toml
[dependencies]
agones = { path = "../agones/sdks/rust" }
```

Add `extern crate agones` to your crate root.

## Usage

To begin working with the SDK, create an instance of it. This function blocks until connection and handshake are made.

```rust
let sdk = agones::Sdk::new()?;
```

To send a [health check](../README.md#health) ping call `sdk.health()`.

```rust
if sdk.health().is_ok() {
    println!("Health ping sent");
}
```

To mark that the [game session is completed](../README.md#shutdown) and the game server should be shut down call `sdk.shutdown()`. 

```rust
if sdk.shutdown().is_err() {
    println!("Could not run Shutdown");
}
```
