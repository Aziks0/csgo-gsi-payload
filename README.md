# csgo-gsi-payload

Data structures for CSGO Game State Integration payload.

## Docs

Documentation is available
[here](https://docs.rs/csgo-gsi-payload/0.1.0/csgo_gsi_payload/).

## Exemple

Using [`axum`](https://github.com/tokio-rs/axum):

```rust
use std::net::SocketAddr;

use axum::{extract::Json, routing::post, Router};

async fn root(Json(payload): Json<csgo_gsi_payload::Payload>) {
    let steam_id_provider = match payload.provider {
        Some(provider) => provider.steam_id,
        None => String::from("unknown"),
    };
    
    println!("SteamID of the provider: {}", steam_id_provider);
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", post(root));
    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

## Builder

Check out [`csgo-gsi-builder`](https://github.com/Aziks0/csgo-gsi-builder) if
you want to build game state integration cfg files.

## Notes

It has not been tested in _Danger Zone_, nor _Hostage Rescue_ modes.
