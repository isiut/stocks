use chrono::{TimeZone, Utc};
use std::time::{Duration, UNIX_EPOCH};
use tokio_test;
use yahoo_finance_api as yahoo;

fn main() {
    let ticker = "BAC";
    let provider = yahoo::YahooConnector::new();
    let response = tokio_test::block_on(provider.get_quote_range(ticker, "1d", "1mo")).unwrap();
    let quotes = response.quotes().unwrap();
    println!("Quotes of the last month for {}: {:?}", ticker, quotes);
}
