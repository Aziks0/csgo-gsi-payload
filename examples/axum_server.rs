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
