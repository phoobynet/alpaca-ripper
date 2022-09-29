extern crate core;

use crate::alpaca::trade::trade_client::TradeClient;

mod alpaca;
mod options;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let trade_client = TradeClient::new();
    let account = trade_client.account().await?;

    println!("{:?}", account);
    Ok(())
}
