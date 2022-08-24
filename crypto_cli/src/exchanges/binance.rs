use super::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct Ticker {
    symbol: String,
    priceChange: String,
    priceChangePercent: String,
    weightedAvgPrice: String,
    prevClosePrice: String,
    lastPrice: String,
    lastQty: String,
    bidPrice: String,
    bidQty: String,
    askPrice: String,
    askQty: String,
    openPrice: String,
    highPrice: String,
    lowPrice: String,
    volume: String,
    quoteVolume: String,
    openTime: u64,
    closeTime: u64,
    firstId: u128,
    lastId: u128,
    count: u32,
}

impl Ticker {
    fn to_instrument(&self) -> Instrument {
        Instrument {
            exName: "Binance".to_string(),
            symbol: self.symbol.clone(),
            price: self.lastPrice.parse::<f32>().unwrap_or(0f32),
            price_24h_change_percent: self.priceChangePercent.parse::<f32>().unwrap_or(0f32),
        }
    }
}

pub struct Binance;

impl Exchange for Binance {
    fn get_instrument(symbol: &str) -> Result<Instrument, Box<dyn std::error::Error>> {
        let ticker = HTTP_CLIENT
            .get("https://api.binance.com/api/v3/ticker/24hr")
            .query(&[("symbol", symbol)])
            .send()?
            .json::<Ticker>()?;

        Ok(ticker.to_instrument())
    }

    fn get_instruments(symbols: Vec<&str>) -> Result<Vec<Instrument>, Box<dyn std::error::Error>> {
        let tickers = HTTP_CLIENT
            .get("https://api.binance.com/api/v3/ticker/24hr")
            .query(&[("symbols", format!("{:?}", symbols).replace(" ", ""))])
            .send()?
            .json::<Vec<Ticker>>()?;

        Ok(tickers.iter().map(|t| t.to_instrument()).collect())
    }
}
