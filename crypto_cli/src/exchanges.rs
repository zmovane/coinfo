use once_cell::sync::Lazy;
use std::{env, error::Error};
mod binance;
pub use self::binance::Binance;
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
    let builder = reqwest::blocking::Client::builder();
    let builder = if let Ok(url) = env::var("HTTP_PROXY") {
        builder.proxy(reqwest::Proxy::https(url).ok().unwrap())
    } else {
        builder
    };
    builder.build().ok().unwrap()
});
