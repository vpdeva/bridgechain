
# BridgeChain

BridgeChain is a lightweight Rust library designed for blockchain interoperability, enabling seamless token transfers and chain connections.

## Features
- Connect to multiple blockchain networks (Ethereum, Polygon).
- Transfer tokens between chains with ease.
- Utilities for validation and common operations.

## Usage
### Connect to a Blockchain
```rust
use bridgechain::chains::ethereum::connect_ethereum;

fn main() {
    println!("{}", connect_ethereum());
}
```

### Transfer Tokens
```rust
use bridgechain::transfer_token;

fn main() {
    match transfer_token("Ethereum", "Polygon", 100.0) {
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("Error: {}", e),
    }
}
```

## Testing
Run tests using:
```
cargo test
```
    