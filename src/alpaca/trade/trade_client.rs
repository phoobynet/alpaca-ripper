use crate::alpaca::client::get_client;
use crate::alpaca::trade::account::Account;
use reqwest::{Client, Url};
use serde::de::DeserializeOwned;
use std::error::Error;
use std::fmt::Debug;

pub struct TradeClient {
    client: Client,
    base_url: Url,
}

impl TradeClient {
    pub fn new() -> Self {
        Self {
            base_url: Url::parse("https://paper-api.alpaca.markets").unwrap(),
            client: get_client().unwrap(),
        }
    }

    pub async fn account(&self) -> Result<Account, Box<dyn Error>> {
        // TODO: optimize this
        let account_url = self.base_url.join("/v2/account")?;
        let account = self.get::<Account>(account_url.as_str()).await?;

        Ok(account)
    }

    async fn get<T: DeserializeOwned + Debug>(&self, url: &str) -> Result<T, Box<dyn Error>> {
        println!("URL: {}", url);

        let response = self.client.get(url).send().await?;

        match response.status() {
            reqwest::StatusCode::NOT_FOUND => {
                panic!("NOT FOUND")
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                panic!("UNAUTHORIZED: check your environment credentials")
            }
            reqwest::StatusCode::OK => (),
            _ => {
                panic!("Unexpected status {}", response.status())
            }
        }

        let result = match response.json::<T>().await {
            Ok(result) => result,
            Err(e) => {
                println!("{}", e);
                panic!("There was a problem decoding the data")
            }
        };

        Ok(result)
    }
}
