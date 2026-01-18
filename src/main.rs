use matchbook::Orderbook;
use matchstick::adapter::process_message;
use matchstick::kraken::client::KrakenClient;
use matchstick::messages::Response;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let mut client = KrakenClient::new().await?;
    let mut orderbook = Orderbook::new();

    loop {
        let msg = client.read().await?;

        if let Ok(message) = serde_json::from_str::<Response>(&msg) {
            match process_message(&mut orderbook, message) {
                Ok(()) => println!("Processed message"),
                Err(e) => println!("Skip: {}", e),
            }
        } else {
            println!("Other message: {}", msg);
        }
    }
}
