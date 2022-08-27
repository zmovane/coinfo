use std::{collections::HashMap, error::Error};

use serde::Deserialize;
mod coingecko;
mod coinmarketcap;
pub use coingecko::Coingecko;
pub use coinmarketcap::Coinmarketcap;

#[derive(Deserialize)]
pub struct CommunityInfo {
    pub id: String,
    pub symbol: String,
    pub description: String,
    pub homepage: Vec<String>,
    pub explorer: Vec<String>,
    pub forum: Vec<String>,
    pub opensource: Vec<String>,
}

#[derive(Deserialize)]
pub struct AirdropInfo {
    pub project_name: String,
    pub symbol: String,
    pub participated: String,
    pub number_of_winners: u32,
    pub total_airdrop_amount: String,
    pub start_date: i64,
    pub end_date: i64,
}

// https://coinmarketcap.com/airdrop/ongoing/

pub trait Aggregator {
    fn get_community_info(&self, currency: String) -> Result<CommunityInfo, Box<dyn Error>>;
    fn get_airdrops(&self, status: String) -> Result<Vec<AirdropInfo>, Box<dyn Error>>;
}
