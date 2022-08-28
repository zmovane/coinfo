use crate::{
    commands::TickerArg,
    display::display_tickers,
    exchanges::{Binance, Exchange, SymbolFormatter, Ticker, OKX},
};
use std::error::Error;

pub fn get_tickers_by_exchange(
    ex_arg: String,
    currencies: Vec<&str>,
) -> Result<Vec<Ticker>, Box<dyn Error>> {
    match ex_arg.as_str() {
        "okx" => get_tickers(OKX, currencies),
        _ => get_tickers(Binance, currencies),
    }
}

pub fn get_tickers<T: Exchange + SymbolFormatter>(
    ex: T,
    currencies: Vec<&str>,
) -> Result<Vec<Ticker>, Box<dyn Error>> {
    let symbols: Vec<String> = currencies
        .iter()
        .map(|&i| ex.format_symbol(i.to_string()))
        .collect();
    ex.get_tickers(symbols)
}

pub fn handle_ticker(ticker_arg: TickerArg) {
    let currencies: Vec<&str> = ticker_arg.currencies.split(",").map(|i| i.trim()).collect();
    let ex_arg = ticker_arg
        .exchange
        .unwrap_or("Binance".to_string())
        .to_lowercase();
    match get_tickers_by_exchange(ex_arg, currencies) {
        Ok(data) => {
            display_tickers(data);
        }
        Err(e) => eprintln!("{}", e),
    }
}
