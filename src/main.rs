use axum::routing::get;
use serde::Deserialize;
use socketioxide::{extract::{SocketRef, Data as SocketioxideData}, SocketIo};

use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

#[derive(Debug, Deserialize)]
struct MessageIn {
    room: String,
    text: String,
}

async fn on_connect(socket: SocketRef) {
    info!("socket connected {}", socket.id);

    socket.on("message", |_: SocketRef, SocketioxideData::<MessageIn>(data) | {
        info!("Received message {:?}", data)
    })
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    let (socketio_layer, io) = SocketIo::new_layer();

    io.ns("/", on_connect);

    let cors = CorsLayer::permissive();

    let app = axum::Router::<_>::new()
        .route("/", get(|| async { "heartbeat\n" }))
        .layer(socketio_layer)
        .layer(cors);
    // Bottom-up

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();

    info!("Server started");
    Ok(())
}
