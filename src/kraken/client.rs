use crate::messages::{SubscribeParams, SubscribeRequest, TokenResponse};
use base64::{Engine as _, engine::general_purpose};
use futures_util::SinkExt;
use hmac::{Hmac, Mac};
use serde::Serialize;
use sha2::{Digest, Sha256, Sha512};
use std::env;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::{Message, Utf8Bytes};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};

type KrakenStream = WebSocketStream<MaybeTlsStream<TcpStream>>;
type HmacSha512 = Hmac<Sha512>;

#[derive(Debug, thiserror::Error)]
pub enum ConnectionError {
    #[error("Failed to connect to Kraken's API")]
    FailedToConnect(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("Missing API Key")]
    MissingApiKey,
    #[error("Invalid Request")]
    InvalidRequest(#[from] reqwest::Error),
    #[error("Missing Token")]
    MissingToken,
    #[error("API Error: {0:?}")]
    ApiError(String),
}

#[allow(dead_code)]
pub struct KrakenClient {
    stream: KrakenStream,
    url: String,
}

#[derive(Serialize)]
pub struct TokenRequest {
    nonce: u64,
}

impl KrakenClient {
    fn kraken_api_sign(path: &str, nonce: &str, postdata: &str, api_secret: &str) -> String {
        // 1. SHA256(nonce + postdata)
        let mut sha256 = Sha256::new();
        sha256.update(nonce.as_bytes());
        sha256.update(postdata.as_bytes());
        let hash = sha256.finalize();

        // 2. path + hash
        let mut message = Vec::with_capacity(path.len() + hash.len());
        message.extend_from_slice(path.as_bytes());
        message.extend_from_slice(&hash);

        // 3. HMAC-SHA512
        let secret = general_purpose::STANDARD
            .decode(api_secret)
            .expect("invalid base64 secret");

        let mut mac = HmacSha512::new_from_slice(&secret).expect("HMAC can take key of any size");
        mac.update(&message);

        // 4. base64 encode
        general_purpose::STANDARD.encode(mac.finalize().into_bytes())
    }

    pub async fn new() -> Result<KrakenClient, ConnectionError> {
        let api_key = env::var("KRAKEN_API_KEY").map_err(|_| ConnectionError::MissingApiKey)?;
        let api_secret =
            env::var("KRAKEN_PRIVATE_KEY").map_err(|_| ConnectionError::MissingApiKey)?;
        let nonce = chrono::Utc::now().timestamp_millis().to_string();
        let postdata = format!("nonce={}", nonce);
        let url = "https://api.kraken.com/0/private/GetWebSocketsToken";
        let path = "https://api.kraken.com/0/private/GetWebSocketsToken";
        let ws = "wss://ws-l3.kraken.com/v2";

        let api_sign = Self::kraken_api_sign(path, &nonce, &postdata, &api_secret);
        let token = Self::get_ws_token(url, api_key, api_sign, postdata)
            .await
            .unwrap();

        let stream = Self::connect_with_token(ws, token).await?;
        Ok(KrakenClient {
            stream,
            url: ws.to_string(),
        })
    }

    async fn get_ws_token(
        url: &str,
        api_key: String,
        api_sign: String,
        postdata: String,
    ) -> Result<String, ConnectionError> {
        let client = reqwest::Client::new();
        let response = client
            .post(url)
            .header("API-Key", api_key)
            .header("API-Sign", api_sign)
            .header("Content-Type", "x-www-form-urlencoded")
            .header("Accept", "application/json")
            .body(postdata)
            .send()
            .await?;

        let token: TokenResponse = response
            .json()
            .await
            .map_err(ConnectionError::InvalidRequest)?;

        if !token.error.is_empty() {
            return Err(ConnectionError::ApiError(token.error.join(", ")));
        }
        let result = token.result.ok_or(ConnectionError::MissingToken)?;
        Ok(result.token)
    }

    async fn connect_with_token(url: &str, token: String) -> Result<KrakenStream, ConnectionError> {
        let request = url.into_client_request().unwrap();
        let (mut stream, _) = connect_async(request).await?;

        let params = SubscribeParams {
            channel: "level3".to_string(),
            symbol: vec!["BTC/USD".to_string()],
            depth: Some(1000),
            snapshot: Some(true),
            token,
            req_id: None,
        };

        let body = SubscribeRequest {
            method: "subscribe".to_string(),
            params,
        };

        let msg = serde_json::to_string(&body).unwrap();
        stream.send(Message::Text(Utf8Bytes::from(msg))).await?;

        Ok(stream)
    }
}
