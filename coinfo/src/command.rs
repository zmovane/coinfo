use clap::{Parser, Subcommand};

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
    Price { currencies: String },
    #[clap(arg_required_else_help = true)]
    Info { currency: String },
}
