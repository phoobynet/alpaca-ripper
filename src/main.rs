use crate::alpaca::trade::account::get_account;

mod alpaca;
mod options;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let account = get_account().await.unwrap();

    println!("{:?}", account);
    Ok(())
}
