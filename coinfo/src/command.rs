use clap::{Args, Parser, Subcommand};
use once_cell::sync::Lazy;

#[derive(Debug, Parser)]
#[clap(name = "coinfo")]
#[clap(about = "A CLI tool for everything about cryptocurrencies")]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[clap(arg_required_else_help = true)]
    Ticker(TickerArg),
    #[clap(arg_required_else_help = true)]
    Community { currency: String },
}

#[derive(Debug, Args)]
pub struct TickerArg {
    pub currencies: String,
    #[clap(short, long)]
    pub exchange: Option<String>,
}

pub static DEFAULT_EXCHANGE: Lazy<String> = Lazy::new(|| String::from("Binance"));
