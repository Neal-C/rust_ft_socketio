mod state;

use axum::extract::State as AxumState;
use axum::routing::get;
use serde::{Deserialize, Serialize};
use socketioxide::{
    extract::{Data as SocketioxideData, SocketRef, State as SocketioxideState},
    SocketIo,
};

use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

#[derive(Debug, Deserialize)]
struct MessageIn {
    room: String,
    text: String,
}

#[derive(Serialize)]
struct Messages {
    messages: Vec<state::Message>,
}

async fn handler_hello(AxumState(io): AxumState<SocketIo>) {
    let _ = io.emit("hello", "goodbye");
}

async fn on_connect(socket: SocketRef) {
    info!("socket connected {}", socket.id);

    socket.on(
        "join",
        |socket_ref: SocketRef,
         SocketioxideData::<String>(room),
         msg_store: SocketioxideState<state::MessageStore>| async move {
            info!("Received join event {:?}", room);

            let _ = socket_ref.leave_all();
            let _ = socket_ref.join(room.clone());
            let messages = msg_store.get(&room).await;
            let _ = socket_ref.emit("messages", Messages { messages });
        },
    );

    socket.on(
        "message",
        |socket_ref: SocketRef,
         SocketioxideData::<MessageIn>(data),
         msg_store: SocketioxideState<state::MessageStore>| async move {
            info!("Received message {:?}", data);

            let response = state::Message {
                text: data.text,
                user: format!("anonymous-{}", socket_ref.id),
                date: chrono::Utc::now(),
            };

            msg_store.insert(&data.room, response.clone()).await;

            let _ = socket_ref.within(data.room).emit("message", response);
        },
    )
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    let message_store = state::MessageStore::default();

    let (socketio_layer, io) = SocketIo::builder().with_state(message_store).build_layer();

    io.ns("/", on_connect);

    let cors = CorsLayer::permissive();

    let app = axum::Router::<_>::new()
        .route("/", get(|| async { "heartbeat\n" }))
        .route("/hello", get(handler_hello))
        .with_state(io)
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
