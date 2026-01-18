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
    expires: u64,
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

#[derive(Deserialize)]
pub struct Response {
    pub channel: String,
    #[serde(rename = "type")]
    pub message_type: String,
    pub data: Vec<Data>,
}

#[derive(Deserialize)]
pub struct Data {
    #[serde(default)]
    pub checksum: Option<u32>,
    #[serde(default)]
    pub symbol: Option<String>,
    #[serde(default)]
    pub timestamp: Option<String>,
    #[serde(default)]
    pub bids: Vec<OrderEvent>,
    #[serde(default)]
    pub asks: Vec<OrderEvent>,
}

#[derive(Deserialize)]
pub struct OrderEvent {
    #[serde(default = "default_event")]
    pub event: EventType,
    pub order_id: String,
    pub limit_price: f64,
    pub order_qty: f64,
    pub timestamp: String,
}

#[derive(Deserialize)]
pub enum EventType {
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "modify")]
    Modify,
    #[serde(rename = "delete")]
    Delete,
}

fn default_event() -> EventType {
    EventType::Add
}
