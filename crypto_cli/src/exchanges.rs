use once_cell::sync::Lazy;
use std::error::Error;
pub mod binance;

#[derive(Debug)]
pub struct Instrument {
    exName: String,
    symbol: String,
    price: f32,
    price_24h_change_percent: f32,
}

pub trait Exchange {
    fn get_instrument(symbol: &str) -> Result<Instrument, Box<dyn Error>>;
    fn get_instruments(symbols: Vec<&str>) -> Result<Vec<Instrument>, Box<dyn Error>>;
}

pub static HTTP_CLIENT: Lazy<reqwest::blocking::Client> = Lazy::new(|| {
    reqwest::blocking::Client::builder()
        .proxy(reqwest::Proxy::https("http://127.0.0.1:7890").ok().unwrap())
        .build()
        .ok()
        .unwrap()
});
