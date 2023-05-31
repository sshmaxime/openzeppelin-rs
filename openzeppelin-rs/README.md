# openzeppelin-rs

## Quickstart

> This library depends on `ethers-rs`. If you haven't already added it to your project you can do it with `cargo add ethers`.

Add `openzeppelin-rs` to your project:

```
cargo add openzeppelin-rs
```

And add this to your code:

```
use openzeppelin_rs::*;
```

You are good to go !

## Exemple

```rust
use ethers::types::Address;
use openzeppelin_rs::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let address: Address = WETH_ADDRESS.parse()?;
    let contract = ERC20::new(address, **yourProvider**);

    println!("{}", contract.symbol().await?);

    Ok(())
}
```
