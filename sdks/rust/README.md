# Rust Game Server Client SDK

This is the Rust version of the Agones Game Server Client SDK. 
Check the [Client SDK Documentation](../) for more details on each of the SDK functions and how to run the SDK locally.

## Usage

```toml
[dependencies]
agones = { path = "../agones/sdks/rust" }
```

```rust
extern crate agones;
```

```rust
let sdk = agones::Sdk::new()?;
```

```rust
sdk.health()?;
```

```shutdown
sdk.shutdown()?;
```
