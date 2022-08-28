use crate::{
    aggregators::{self, AirdropInfo, Coinmarketcap},
    commands::{self, AirdropArg},
    display::display_airdrops,
};
use std::error::Error;

pub fn get_airdrops<T: aggregators::Aggregator>(
    aggregator: T,
    status: String,
) -> Result<Vec<AirdropInfo>, Box<dyn Error>> {
    aggregator.get_airdrops(status)
}

pub fn handle_aridrop(airdrop_arg: AirdropArg) {
    let status = airdrop_arg.status.unwrap_or(commands::Status::Ongoing);
    match get_airdrops(Coinmarketcap, status.to_string()) {
        Ok(data) => {
            display_airdrops(data);
        }
        Err(e) => eprintln!("{}", e),
    }
}
