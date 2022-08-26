use std::{collections::HashMap, error::Error};

use serde::Deserialize;
mod coingecko;
pub use coingecko::Coingecko;

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

pub trait Aggregator {
    fn get_community_info(&self, currency: String) -> Result<CommunityInfo, Box<dyn Error>>;
}
