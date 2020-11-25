
# nanowrimo

[![crates.io](https://img.shields.io/crates/v/nanowrimo.svg)](https://crates.io/crates/nanowrimo)
[![Documentation](https://docs.rs/nanowrimo/badge.svg)](https://docs.rs/nanowrimo)
[![MIT/Apache-2 licensed](https://img.shields.io/crates/l/nanowrimo.svg)](./LICENSE-APACHE)

An easy-to use NanoWrimo API client for Rust

- Built on reqwest
- Supports both anonymous and logged-in access

## Example

This example uses [Tokio](https://tokio.rs)

```toml
[dependencies]
nanowrimo = "0.1"
tokio = { version = "0.2", features = ["full"] }
```

```rust,no_run
use nanowrimo::{NanoClient, ObjectData};

#[tokio::main]
async fn main() {
    let client = NanoClient::new_user("username", env!("NANO_PASSWORD"))
        .await
        .expect("Couldn't create logged in NanoClient");
        
    let user = client.current_user()
        .await
        .expect("Couldn't get current user")
        .data;
        
    println!("User ID: {}", user.id)
    if let ObjectData::User(data) = user.data {
        println!("User Bio: {}", data.bio);
        println!("Avatar: {}", data.avatar);
    }
}
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
