use once_cell::sync::Lazy;
use std::{env, error::Error};
mod binance;
pub use self::binance::Binance;
#[derive(Debug)]
pub struct Instrument {
    pub exName: String,
    pub symbol: String,
    pub price: f32,
    pub price_24h_change_percent: f32,
    pub volume: f64,
}

pub trait Exchange {
    fn get_instrument(&self, symbol: String) -> Result<Instrument, Box<dyn Error>>;
    fn get_instruments(&self, symbols: Vec<String>) -> Result<Vec<Instrument>, Box<dyn Error>>;
}

pub trait SymbolFormatter {
    fn format_symbol(&self, base: String) -> String;
}

pub static QUOTE: Lazy<String> =
    Lazy::new(|| env::var("COINFO_QUOTE").unwrap_or(String::from("USDT")));

pub static HTTP_CLIENT: Lazy<reqwest::blocking::Client> = Lazy::new(|| {
    let builder = reqwest::blocking::Client::builder();
    let builder = if let Ok(url) = env::var("HTTP_PROXY") {
        builder.proxy(reqwest::Proxy::https(url).ok().unwrap())
    } else {
        builder
    };
    builder.build().ok().unwrap()
});
