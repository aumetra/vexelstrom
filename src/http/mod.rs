use axum::serve::ListenerExt;
use tokio::net::TcpListener;

use crate::config::Configuration;

pub mod router;
pub mod routes;

pub async fn serve(config: Configuration) -> eyre::Result<()> {
    let router = self::router::construct();

    let listener = TcpListener::bind((config.server.interface, config.server.port)).await?;
    let listener = listener.tap_io(|tcp_stream| {
        if let Err(err) = tcp_stream.set_nodelay(true) {
            trace!("failed to set TCP_NODELAY on incoming connection: {err:#}");
        }
    });

    axum::serve(listener, router).await?;

    Ok(())
}
