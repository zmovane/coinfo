use aggregators::{Coingecko, CommunityInfo};
use clap::Parser;
use command::Commands;
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
        Commands::Ticker { currencies } => {
            let currencies: Vec<&str> = currencies.split(",").map(|i| i.trim()).collect();
            match get_tickers(OKX, currencies) {
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
