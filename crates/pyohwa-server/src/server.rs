use std::path::Path;

use axum::extract::ws::{Message, WebSocket};
use axum::extract::WebSocketUpgrade;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tower_http::services::ServeDir;

use crate::reload;

/// Start the HTTP + WebSocket server.
pub async fn start_server(
    project_root: &Path,
    port: u16,
    reload_tx: broadcast::Sender<()>,
) -> Result<(), crate::error::ServerError> {
    let output_dir = {
        let config = pyohwa_core::config::load(project_root)
            .map_err(|e| crate::error::ServerError::Server(e.to_string()))?;
        project_root.join(&config.build.output_dir)
    };

    let app = Router::new()
        .route(reload::WS_PATH, get(move |ws| ws_handler(ws, reload_tx)))
        .fallback_service(ServeDir::new(&output_dir).append_index_html_on_directories(true));

    let addr = format!("0.0.0.0:{port}");
    let listener = TcpListener::bind(&addr)
        .await
        .map_err(|e| crate::error::ServerError::Server(format!("failed to bind {addr}: {e}")))?;

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .map_err(|e| crate::error::ServerError::Server(e.to_string()))?;

    Ok(())
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    reload_tx: broadcast::Sender<()>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_ws(socket, reload_tx))
}

async fn handle_ws(mut socket: WebSocket, reload_tx: broadcast::Sender<()>) {
    let mut rx = reload_tx.subscribe();

    loop {
        tokio::select! {
            result = rx.recv() => {
                match result {
                    Ok(()) => {
                        if socket
                            .send(Message::Text(reload::RELOAD_MESSAGE.into()))
                            .await
                            .is_err()
                        {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
            msg = socket.recv() => {
                match msg {
                    Some(Ok(_)) => {} // ignore client messages
                    _ => break,       // client disconnected
                }
            }
        }
    }
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install Ctrl+C handler");
    eprintln!("\nShutting down...");
}
