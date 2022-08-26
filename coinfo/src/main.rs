use aggregators::{Coingecko, CommunityInfo};
use clap::Parser;
use command::{Commands, DEFAULT_EXCHANGE};
use display::{display_community_info, display_tickers};
use exchanges::{Binance, Exchange, SymbolFormatter, Ticker, OKX};
use std::error::Error;
mod aggregators;
mod command;
mod display;
mod exchanges;

fn main() {
    let args = command::Cli::parse();
    match args.command {
        Commands::Ticker(ticker) => {
            let currencies: Vec<&str> = ticker.currencies.split(",").map(|i| i.trim()).collect();
            let ex_arg = ticker
                .exchange
                .unwrap_or(DEFAULT_EXCHANGE.to_string())
                .to_lowercase();
            match get_tickers_by_exchange(ex_arg, currencies) {
                Ok(data) => {
                    display_tickers(data);
                }
                Err(e) => eprintln!("{}", e),
            }
        }
        Commands::Community { currency } => match get_community_info(Coingecko, currency) {
            Ok(data) => {
                display_community_info(data);
            }
            Err(e) => eprintln!("{}", e),
        },
    }
}

fn get_tickers_by_exchange(
    ex_arg: String,
    currencies: Vec<&str>,
) -> Result<Vec<Ticker>, Box<dyn Error>> {
    match ex_arg.as_str() {
        "okx" => get_tickers(OKX, currencies),
        _ => get_tickers(Binance, currencies),
    }
}

fn get_community_info<T: aggregators::Aggregator>(
    aggregator: T,
    currency: String,
) -> Result<CommunityInfo, Box<dyn Error>> {
    aggregator.get_community_info(currency)
}

fn get_tickers<T: Exchange + SymbolFormatter>(
    ex: T,
    currencies: Vec<&str>,
) -> Result<Vec<Ticker>, Box<dyn Error>> {
    let symbols: Vec<String> = currencies
        .iter()
        .map(|&i| ex.format_symbol(i.to_string()))
        .collect();
    ex.get_tickers(symbols)
}
