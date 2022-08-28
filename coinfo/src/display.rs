use crate::{
    aggregators::{AirdropInfo, CommunityInfo},
    exchanges::{Ticker, QUOTE},
};
use chrono::NaiveDateTime;
use comfy_table::{ContentArrangement, Table};

pub fn display_tickers(tickers: Vec<Ticker>) {
    let mut table = Table::new();
    table.set_header(vec![
        "Symbol",
        "Exchange",
        &format!("Price ({})", QUOTE.to_string()),
        "PriceChange%",
        &format!("24H Volume ({})", QUOTE.to_string()),
    ]);
    for ticker in tickers.iter() {
        table.add_row(vec![
            ticker.symbol.clone(),
            ticker.ex_name.clone(),
            ticker.price.to_string(),
            ticker.price_24h_change_percent.to_string(),
            ticker.volume.to_string(),
        ]);
    }
    println!("{table}");
}

pub fn display_community_info(info: CommunityInfo) {
    let mut table = Table::new();
    table.set_content_arrangement(ContentArrangement::Dynamic);

    table.set_header(vec![
        "Currency",
        "Homepage",
        "Explorer",
        "Opensource",
        "Description",
    ]);
    let readmore = format!("https://www.coingecko.com/en/coins/{}", info.id);
    let description = info
        .description
        .split(" ")
        .take(50)
        .collect::<Vec<&str>>()
        .join(" ");

    table.add_row(vec![
        info.symbol,
        info.homepage.join("\n"),
        info.explorer.join("\n"),
        info.opensource.join("\n"),
        format!("{}...\n\nRead more on {}", description, readmore),
    ]);
    println!("{table}")
}

fn format_date(secs: i64) -> String {
    NaiveDateTime::from_timestamp(secs, 0)
        .format("%Y-%m-%d %H:%M:%S")
        .to_string()
}

pub fn display_airdrops(airdrops: Vec<AirdropInfo>) {
    let mut table = Table::new();
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec![
        "Ongoing Project",
        "Currency",
        "Participated",
        "Number of winners",
        "Total airdrop amount",
        "Start date (UTC)",
        "End date (UTC)",
        "Link",
    ]);

    for airdrop in airdrops {
        table.add_row(vec![
            airdrop.project_name,
            airdrop.symbol.clone(),
            airdrop.participated.to_string(),
            airdrop.number_of_winners.to_string(),
            airdrop.total_airdrop_amount.to_string(),
            format_date(airdrop.start_date / 1000),
            format_date(airdrop.end_date / 1000),
            format!(
                "https://coinmarketcap.com/currencies/{}/airdrop/",
                airdrop.symbol.to_lowercase()
            ),
        ]);
    }
    println!("{table}")
}
