use crate::{
    aggregators::CommunityInfo,
    exchanges::{Instrument, QUOTE},
};
use prettytable::{row, Table};

pub fn display_instruments(instruments: Vec<Instrument>) {
    let mut table = Table::new();
    table.set_titles(row![
        "Symbol",
        "Exchange",
        format!("Price ({})", QUOTE.to_string()),
        "PriceChange%",
        format!("24H Volume ({})", QUOTE.to_string())
    ]);
    for instr in instruments.iter() {
        table.add_row(row![
            instr.symbol,
            instr.ex_name,
            instr.price,
            instr.price_24h_change_percent,
            instr.volume
        ]);
    }
    table.printstd();
}

pub fn display_community_info(info: CommunityInfo) {
    let mut table = Table::new();
    table.set_titles(row![
        "Currency",
        "Homepage",
        "Explorer",
        "Opensource",
        "Description"
    ]);
    let readmore = format!("https://www.coingecko.com/en/coins/{}", info.id);
    let description = info
        .description
        .split(" ")
        .take(50)
        .collect::<Vec<&str>>()
        .join(" ");

    table.add_row(row![
        info.symbol,
        info.homepage.join("\n"),
        info.explorer.join("\n"),
        info.opensource.join("\n"),
        format!("{}\nRead more on {}", description, readmore)
    ]);
    table.printstd();
}
