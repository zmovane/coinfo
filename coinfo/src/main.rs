use clap::Parser;
use command::Commands;
use exchanges::{Binance, Exchange};
mod command;
mod exchanges;

fn main() {
    let args = command::Cli::parse();
    match args.command {
        Commands::Price { currencies } => {
            let currencies: Vec<&str> = currencies.split(",").collect();
            println!("{:?}", currencies)
        }
    }
    // match Binance::get_instrument("BTCUSDT") {
    //     Ok(data) => {
    //         println!("{:?}", data);
    //     }
    //     Err(e) => {
    //         println!("{:?}", e);
    //     }
    // }
    // match Binance::get_instruments(vec!["BTCUSDT", "ETHUSDT"]) {
    //     Ok(data) => {
    //         println!("{:?}", data);
    //     }
    //     Err(e) => {
    //         println!("{:?}", e);
    //     }
    // }
}
