use super::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Response<T> {
    code: String,
    msg: String,
    data: Vec<T>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Ticker {
    inst_type: String,
    inst_id: String,
    last: String,
    last_sz: String,
    ask_px: String,
    ask_sz: String,
    bid_px: String,
    bid_sz: String,
    open24h: String,
    high24h: String,
    low24h: String,
    vol_ccy24h: String,
    vol24h: String,
    ts: String,
    sod_utc0: String,
    sod_utc8: String,
}

impl Ticker {
    fn converted(&self) -> super::Ticker {
        let open = self.open24h.parse::<f32>().unwrap();
        let price_change = (self.last.parse::<f32>().unwrap() - open) / open * 100.0;
        super::Ticker {
            ex_name: "OKX".to_string(),
            symbol: self.inst_id.clone(),
            price: self.last.parse::<f32>().unwrap_or(0f32),
            price_24h_change_percent: price_change,
            volume: self.vol_ccy24h.parse::<f64>().unwrap_or(0f64),
        }
    }
}

pub struct OKX;

impl SymbolFormatter for OKX {
    fn format_symbol(&self, base: String) -> String {
        format!("{}-{}", base, QUOTE.to_string())
    }
}

impl Exchange for OKX {
    fn get_ticker(&self, symbol: String) -> Result<super::Ticker, Box<dyn std::error::Error>> {
        let response = HTTP_CLIENT
            .get("https://www.okx.com/api/v5/market/ticker")
            .query(&[("instId", symbol)])
            .send()?
            .json::<Response<Ticker>>()?;
        Ok(response.data.get(0).unwrap().converted())
    }

    fn get_tickers(
        &self,
        symbols: Vec<String>,
    ) -> Result<Vec<super::Ticker>, Box<dyn std::error::Error>> {
        let tickers: Vec<super::Ticker> = symbols
            .iter()
            .map(|s| self.get_ticker(s.to_string()))
            .filter(|ticker| if let Ok(_) = ticker { true } else { false })
            .map(|ticker| ticker.unwrap())
            .collect();

        Ok(tickers)
    }
}
