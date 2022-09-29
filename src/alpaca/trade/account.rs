use crate::alpaca::trade::client::get_client;
use serde::Deserialize;

pub async fn get_account() -> Result<Account, &'static str> {
    let client = get_client().unwrap();

    let account = client
        .get("https://paper-api.alpaca.markets/v2/account")
        .send()
        .await
        .unwrap()
        .json::<Account>()
        .await
        .unwrap();

    Ok(account)
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Account {
    account_blocked: bool,
    account_number: String,
    buying_power: String,
    cash: String,
    created_at: String,
    currency: String,
    crypto_status: String,
    non_marginable_buying_power: String,
    accrued_fees: String,
    pending_transfer_in: String,
    pending_transfer_out: Option<String>,
    daytrade_count: u8,
    daytrading_buying_power: String,
    equity: String,
    id: String,
    initial_margin: String,
    last_equity: String,
    last_maintenance_margin: String,
    long_market_value: String,
    maintenance_margin: String,
    multiplier: String,
    pattern_day_trader: bool,
    portfolio_value: String,
    regt_buying_power: String,
    short_market_value: String,
    shorting_enabled: bool,
    sma: String,
    status: String,
    trade_suspended_by_user: bool,
    trading_blocked: bool,
    transfers_blocked: bool,
    balance_asof: String,
}
