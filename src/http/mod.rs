use crate::config::Configuration;
use axum::serve::ListenerExt;
use state::AppState;
use tokio::net::TcpListener;

pub mod error;
pub mod router;
pub mod routes;
pub mod state;

pub async fn serve(config: Configuration, state: AppState) -> eyre::Result<()> {
    let router = self::router::construct(state);

    let listener = TcpListener::bind((config.server.interface, config.server.port)).await?;
    let listener = listener.tap_io(|tcp_stream| {
        if let Err(err) = tcp_stream.set_nodelay(true) {
            trace!("failed to set TCP_NODELAY on incoming connection: {err:#}");
        }
    });

    axum::serve(listener, router).await?;

    Ok(())
}
