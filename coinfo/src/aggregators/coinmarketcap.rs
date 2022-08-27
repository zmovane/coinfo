use super::*;
use crate::exchanges::HTTP_CLIENT;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, USER_AGENT};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Response<T> {
    data: T,
}

#[derive(Deserialize, Debug)]
struct AirdropResponse {
    projects: Vec<Airdrop>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Airdrop {
    project_name: String,
    start_date: u64,
    end_date: u64,
    crypto_currency: CryptoCurrency,
    participation_count: String,
    winner_count: u32,
    total_prize: String,
}

#[derive(Deserialize, Debug)]
struct CryptoCurrency {
    symbol: String,
}

impl Airdrop {
    fn as_airdrop_info(&self) -> AirdropInfo {
        AirdropInfo {
            project_name: self.project_name.clone(),
            symbol: self.crypto_currency.symbol.clone(),
            participated: self.participation_count.clone(),
            number_of_winners: self.winner_count,
            total_airdrop_amount: self.total_prize.clone(),
            start_date: self.start_date,
            end_date: self.end_date,
        }
    }
}
pub struct Coinmarketcap;

impl Aggregator for Coinmarketcap {
    fn get_community_info(&self, _: String) -> Result<CommunityInfo, Box<dyn Error>> {
        todo!()
    }

    fn get_airdrops(&self, status: String) -> Result<Vec<AirdropInfo>, Box<dyn Error>> {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/104.0.0.0 Safari/537.36"));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        let result = HTTP_CLIENT
            .get("https://api.coinmarketcap.com/data-api/v3/airdrop/query")
            .headers(headers)
            .query(&[("status", status)])
            .send()?
            .json::<Response<AirdropResponse>>()?;
        Ok(result
            .data
            .projects
            .iter()
            .map(|i| i.as_airdrop_info())
            .collect())
    }
}
