use crate::exchanges::HTTP_CLIENT;

use super::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct SearchResult {
    coins: Vec<Coin>,
}

#[derive(Deserialize, Debug)]
struct Coin {
    id: String,
    name: String,
    api_symbol: String,
    symbol: String,
    market_cap_rank: Option<u32>,
}

#[derive(Deserialize, Debug)]
struct Community {
    id: String,
    symbol: String,
    description: HashMap<String, String>,
    links: Links,
}

#[derive(Deserialize, Debug)]
struct Links {
    homepage: Vec<String>,
    blockchain_site: Vec<String>,
    official_forum_url: Vec<String>,
    repos_url: HashMap<String, Vec<String>>,
}

impl Community {
    fn find_not_empty_urls(&self, urls: Vec<String>) -> Vec<String> {
        urls.iter()
            .filter(|&i| !i.is_empty())
            .map(|i| i.clone())
            .collect()
    }

    fn as_community_info(&self) -> CommunityInfo {
        CommunityInfo {
            id: self.id.clone(),
            symbol: self.symbol.clone(),
            description: self.description.get("en").unwrap().to_string(),
            homepage: self.find_not_empty_urls(self.links.homepage.clone()),
            explorer: self.find_not_empty_urls(self.links.blockchain_site.clone()),
            forum: self.find_not_empty_urls(self.links.official_forum_url.clone()),
            opensource: self.find_not_empty_urls(
                self.links
                    .repos_url
                    .values()
                    .flat_map(|x| x.clone())
                    .collect::<Vec<String>>(),
            ),
        }
    }
}
pub struct Coingecko;

impl Aggregator for Coingecko {
    fn get_community_info(
        &self,
        currency: String,
    ) -> Result<CommunityInfo, Box<dyn std::error::Error>> {
        let searchResult = HTTP_CLIENT
            .get("https://api.coingecko.com/api/v3/search")
            .query(&[("query", currency)])
            .send()?
            .json::<SearchResult>();

        match searchResult {
            Ok(result) => {
                let coin = result
                    .coins
                    .iter()
                    .min_by_key(|i| i.market_cap_rank.unwrap_or(u32::MAX))
                    .unwrap();
                let community = HTTP_CLIENT
                    .get(format!(
                        "https://api.coingecko.com/api/v3/coins/{}",
                        coin.id
                    ))
                    .query(&[
                        ("tickers", false),
                        ("market_data", false),
                        ("community_data", true),
                        ("developer_data", false),
                        ("sparkline", false),
                    ])
                    .send()?
                    .json::<Community>()?;
                Ok(community.as_community_info())
            }
            Err(e) => Err(e.into()),
        }
    }
}
