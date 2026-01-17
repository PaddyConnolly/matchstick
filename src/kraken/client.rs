use std::env;
use thiserror::Error;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};

#[derive(Debug, thiserror::Error)]
pub enum ConnectionError {
    #[error("Failed to connect to Kraken's API")]
    FailedToConnect(#[from] tokio_tungstenite::tungstenite::Error),
}
pub struct KrakenClient {
    stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
    channel: String,
}

impl KrakenClient {
    pub async fn new(channel: String) -> Result<KrakenClient, ConnectionError> {
        let mut request = "wss://ws-l3.kraken.com/v2".into_client_request().unwrap();
        request.headers_mut().insert(
            "api-key",
            HeaderValue::from_str(env::var("KRAKEN_API_KEY").unwrap().as_ref()),
        );

        let (stream, _) = connect_async(request).await?;
        Ok(KrakenClient { stream, channel })
    }
}
