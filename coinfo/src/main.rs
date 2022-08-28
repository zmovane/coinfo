use clap::Parser;
use commands::Commands;
use handlers::{handle_aridrop, handle_command_ticker, handle_community_info};
mod aggregators;
mod commands;
mod display;
mod exchanges;
mod handlers;

fn main() {
    let args = commands::Cli::parse();
    match args.command {
        Commands::Ticker(ticker_arg) => handle_command_ticker(ticker_arg),
        Commands::Community { currency } => handle_community_info(currency),
        Commands::Airdrop(airdrop_arg) => handle_aridrop(airdrop_arg),
    }
}
