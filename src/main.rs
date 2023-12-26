use axum::routing::get;
use socketioxide::{extract::SocketRef, SocketIo};

use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

async fn on_connect(socket: SocketRef) {
    info!("socket connected {}", socket.id)
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
