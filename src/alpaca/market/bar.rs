#![allow(unused_variables, dead_code)]
// https://alpaca.markets/docs/api-references/market-data-api/stock-pricing-data/historical/#bars
use chrono::{DateTime, Utc};
use serde::Deserialize;

pub async fn get_latest_bar(symbol: &str) -> Result<Bar, &'static str> {
    println!("You want symbol {}", symbol);
    Err("Rat farts!")
}

pub async fn get_bars(bars_request: BarsRequest) -> Vec<Bar> {
    // TODO: get_bars
    let mut bars = vec![];
    bars.push(Bar {});

    bars
}

#[derive(Debug, Deserialize)]
pub struct Bar {}

pub enum BarAdjustment {
    Raw,
    Split,
    Dividend,
    All,
}

pub enum BarFeed {
    Sip,
    Iex,
    Otc,
}

pub struct BarsRequest {
    symbol: String,
    timeframe: String,
    start: Option<DateTime<Utc>>,
    end: Option<DateTime<Utc>>,
    limit: Option<u8>,
    page_token: Option<String>,
    adjustment: Option<BarAdjustment>,
    asof: Option<DateTime<Utc>>,
    feed: Option<BarFeed>,
    currency: Option<String>,
}
