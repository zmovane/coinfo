use crate::{
    aggregators::{self, Coingecko, CommunityInfo},
    display::display_community_info,
};
use std::error::Error;

pub fn get_community_info<T: aggregators::Aggregator>(
    aggregator: T,
    currency: String,
) -> Result<CommunityInfo, Box<dyn Error>> {
    aggregator.get_community_info(currency)
}

pub fn handle_community(community_arg: String) {
    match get_community_info(Coingecko, community_arg) {
        Ok(data) => {
            display_community_info(data);
        }
        Err(e) => eprintln!("{}", e),
    }
}
