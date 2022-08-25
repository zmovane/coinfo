use clap::Parser;
use command::Commands;
use display::display_instruments;
use exchanges::{Binance, Exchange, Instrument, SymbolFormatter};
use std::error::Error;
mod command;
mod display;
mod exchanges;

fn main() {
    let args = command::Cli::parse();
    match args.command {
        Commands::Price { currencies } => {
            let currencies: Vec<&str> = currencies.split(",").map(|i| i.trim()).collect();
            match get_instruments(Binance, currencies) {
                Ok(data) => {
                    display_instruments(data);
                }
                Err(e) => eprintln!("{}", e),
            }
        }
    }
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
