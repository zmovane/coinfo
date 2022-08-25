use crate::exchanges::{Instrument, QUOTE};
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
