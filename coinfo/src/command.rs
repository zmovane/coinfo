use clap::{Args, Parser, Subcommand, ValueEnum};
use once_cell::sync::Lazy;
use std::fmt;

#[derive(Debug, Parser)]
#[clap(name = "coinfo")]
#[clap(about = "A CLI tool that provides useful information about cryptocurrencies")]
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
    #[clap(arg_required_else_help = true)]
    Airdrop(AirdropArg),
}

#[derive(Debug, Args)]
pub struct TickerArg {
    pub currencies: String,
    #[clap(short, long)]
    pub exchange: Option<String>,
}

#[derive(Debug, Args)]
pub struct AirdropArg {
    #[clap(short, long)]
    #[clap(value_enum)]
    pub status: Option<Status>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Status {
    Ongoing,
    Upcoming,
    Ended,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub static DEFAULT_EXCHANGE: Lazy<String> = Lazy::new(|| String::from("Binance"));
