use axum::routing::get;
use tracing::info;
use tracing_subscriber::FmtSubscriber;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    let app: axum::Router<_> = axum::Router::<_>::new().route("/", get(|| async { "heartbeat\n" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    info!("Server started");
    Ok(())
}
