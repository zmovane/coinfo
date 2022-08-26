use crate::{
    aggregators::CommunityInfo,
    exchanges::{Instrument, QUOTE},
};
use comfy_table::{ContentArrangement, Table};

pub fn display_instruments(instruments: Vec<Instrument>) {
    let mut table = Table::new();
    table.set_header(vec![
        "Symbol",
        "Exchange",
        &format!("Price ({})", QUOTE.to_string()),
        "PriceChange%",
        &format!("24H Volume ({})", QUOTE.to_string()),
    ]);
    for instr in instruments.iter() {
        table.add_row(vec![
            instr.symbol.clone(),
            instr.ex_name.clone(),
            instr.price.to_string(),
            instr.price_24h_change_percent.to_string(),
            instr.volume.to_string(),
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
