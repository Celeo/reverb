use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Extension,
    },
    handler::get,
    response::IntoResponse,
    AddExtensionLayer, Router,
};
use futures::{sink::SinkExt, stream::StreamExt};
use log::{debug, error, info};
use std::{env, net::SocketAddr, sync::Arc};
use structopt::StructOpt;
use tokio::sync::broadcast;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "reverb",
    about = "Simple websocket broadcast server with rooms"
)]
struct Options {
    #[structopt(short, long)]
    debug: bool,

    #[structopt(short, long)]
    host: Option<String>,

    #[structopt(short, long)]
    port: Option<u64>,
}

struct AppState {
    tx: broadcast::Sender<String>,
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    Extension(state): Extension<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| websocket(socket, state))
}

async fn websocket(stream: WebSocket, state: Arc<AppState>) {
    debug!("Received new connection");
    let (mut sender, mut receiver) = stream.split();

    let mut rx = state.tx.subscribe();
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    let tx = state.tx.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            debug!("Received text: {}", text);
            let _ = tx.send(text);
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
    debug!("Client dropped");
}

#[tokio::main]
async fn main() {
    let options = Options::from_args();
    if options.debug {
        env::set_var("RUST_LOG", "info,reverb=debug");
    } else if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();

    debug!("Setting up");
    let addr: SocketAddr = match format!(
        "{}:{}",
        options.host.unwrap_or_else(|| "127.0.0.1".to_string()),
        options.port.unwrap_or(3000)
    )
    .parse()
    {
        Ok(a) => a,
        Err(e) => {
            error!("Could not parse supplied binding: {}", e);
            return;
        }
    };
    debug!("Bind URL is {}", addr);
    let (tx, _rx) = broadcast::channel(10000);
    let app_state = Arc::new(AppState { tx });

    let app = Router::new()
        .route("/ws", get(websocket_handler))
        .layer(AddExtensionLayer::new(app_state));

    info!("Accepting connections at ws://{}/ws", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    info!("Server stopped");
}
