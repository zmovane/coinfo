use aggregators::{Coingecko, CommunityInfo};
use clap::Parser;
use command::Commands;
use display::{display_community_info, display_instruments};
use exchanges::{Binance, Exchange, Instrument, SymbolFormatter, OKX};
use std::error::Error;
mod aggregators;
mod command;
mod display;
mod exchanges;

fn main() {
    let args = command::Cli::parse();
    match args.command {
        Commands::Price { currencies } => {
            let currencies: Vec<&str> = currencies.split(",").map(|i| i.trim()).collect();
            match get_instruments(OKX, currencies) {
                Ok(data) => {
                    display_instruments(data);
                }
                Err(e) => eprintln!("{}", e),
            }
        }
        Commands::Info { currency } => match get_community_info(Coingecko, currency) {
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

fn get_instruments<T: Exchange + SymbolFormatter>(
    ex: T,
    currencies: Vec<&str>,
) -> Result<Vec<Instrument>, Box<dyn Error>> {
    let symbols: Vec<String> = currencies
        .iter()
        .map(|&i| ex.format_symbol(i.to_string()))
        .collect();
    ex.get_instruments(symbols)
}
