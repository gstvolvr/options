use csv::{ReaderBuilder, WriterBuilder};
use std::env;
use chrono::{DateTime, Utc, NaiveDate, Duration};
pub mod models;
pub mod util;
use models::dividend::Dividend;
use models::options::Options;
use util::calculate_return_after_dividends;
use std::collections::HashMap;
use std::fs::File;

pub mod api;
use api::schwab::call_api;


fn main() {
    // update_returns();
    call_api();
}

fn _process(record: &Options, dividend: &Dividend) -> Option<Options> {

    if record.premium() < 0.05 {
        return None
    }

    let mut options = Options::new(record.clone());

    options.return_after_1_div = Some(calculate_return_after_dividends(&record, dividend, 1));
    options.return_after_2_div = Some(calculate_return_after_dividends(&record, dividend, 2));
    options.return_after_3_div = Some(calculate_return_after_dividends(&record, dividend, 3));
    options.return_after_4_div = Some(calculate_return_after_dividends(&record, dividend, 4));
    options.return_after_5_div = Some(calculate_return_after_dividends(&record, dividend, 5));
    options.return_after_6_div = Some(calculate_return_after_dividends(&record, dividend, 6));

    Some(options)
}


fn update_returns() -> Result<(), Box<dyn std::error::Error>> {
    let mut return_records: Vec<Options> = vec![];
    let option_records: Vec<Options> = util::get_options()?;


    let dividend_map: HashMap<String, Dividend> = match util::get_dividend_map() {
        Ok(map) => {
            map
        },
        Err(e) => {
            println!("Something is up: {}", e);
            return Err(e)
        }
    };

    for record in &option_records {
        let today = Utc::now().date_naive();

        // we only want to process things in the next 20 or so months
        let expiration_date = record.expiration_date();
        if expiration_date > today + Duration::days(30*20) {
            println!("The expiration date is too far out: {:?}", expiration_date);
            continue
        }

        if record.last * 0.50 > record.strike_price {
            println!("The strike price is too high compared to last: {:?} vs {:?}", record.strike_price, record.last);
            continue
        }

        if let Some(dividend) = dividend_map.get(&record.symbol) {
            if let Some(returns) = _process(&record, dividend) {
                // println!("{:?}", returns);
                return_records.push(returns)
            }
        } else {
            // println!("No dividend found for symbol: {:?}", record.symbol);
            continue
        }
    }
    println!("{:?}", return_records.len());

    let file = File::create("../data/rust_returns.csv")?;
    let mut wtr = WriterBuilder::new().from_writer(file);
    for record in &return_records {
        wtr.serialize(record).map_err(|e| {
            println!("Failed to serialize record: {}", e);
            e
        })?;

    }
    wtr.flush()?;

    Ok(())
}

