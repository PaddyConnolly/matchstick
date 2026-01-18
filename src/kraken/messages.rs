use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct TokenResponse {
    pub result: Option<TokenResponseResult>,
    pub error: Vec<String>,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct TokenResponseResult {
    pub token: String,
    expiry: u64,
}

#[derive(Serialize)]
pub struct SubscribeRequest {
    pub method: String,
    pub params: SubscribeParams,
}

#[derive(Serialize)]
pub struct SubscribeParams {
    pub channel: String,
    pub symbol: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<bool>,
    pub token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub req_id: Option<u64>,
}
