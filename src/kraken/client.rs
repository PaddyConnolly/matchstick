use http::HeaderValue;
use std::env;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};

#[derive(Debug, thiserror::Error)]
pub enum ConnectionError {
    #[error("Failed to connect to Kraken's API")]
    FailedToConnect(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("Missing API Key")]
    MissingApiKey,
}

#[allow(dead_code)]
pub struct KrakenClient {
    stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
    url: String,
}

impl KrakenClient {
    pub async fn new(url: String) -> Result<KrakenClient, ConnectionError> {
        let mut request = url.clone().into_client_request().unwrap();
        request.headers_mut().insert(
            "api-key",
            HeaderValue::from_str(env::var("KRAKEN_API_KEY").unwrap().as_ref())
                .ok()
                .unwrap(),
        );

        let (stream, _) = connect_async(request).await?;
        Ok(KrakenClient { stream, url })
    }
}
