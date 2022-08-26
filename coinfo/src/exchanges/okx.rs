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
    fn to_instrument(&self) -> Instrument {
        let open = self.open24h.parse::<f32>().unwrap();
        let price_change = (self.last.parse::<f32>().unwrap() - open) / open * 100.0;
        Instrument {
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
    fn get_instrument(&self, symbol: String) -> Result<Instrument, Box<dyn std::error::Error>> {
        let response = HTTP_CLIENT
            .get("https://www.okx.com/api/v5/market/ticker")
            .query(&[("instId", symbol)])
            .send()?
            .json::<Response<Ticker>>()?;
        Ok(response.data.get(0).unwrap().to_instrument())
    }

    fn get_instruments(
        &self,
        symbols: Vec<String>,
    ) -> Result<Vec<Instrument>, Box<dyn std::error::Error>> {
        let tickers: Vec<Instrument> = symbols
            .iter()
            .map(|s| self.get_instrument(s.to_string()))
            .filter(|instr| if let Ok(_) = instr { true } else { false })
            .map(|instr| instr.unwrap())
            .collect();

        Ok(tickers)
    }
}
