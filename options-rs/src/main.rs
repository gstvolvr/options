use csv::WriterBuilder;
use chrono::{Duration, Utc};
// use crate::models::dividend::Dividend;
// use crate::models::options::Options;
// use options_rs::utils::calculate_return_after_dividends;
use options_rs::test_utils;
use options_rs::api;
use std::collections::HashMap;
use std::fs::File;
use std::option;
use options_rs::api::schwab::{chains, quote};
use options_rs::api::token_storage::TOKEN_STORAGE;

#[tokio::main]
async fn main() {
    println!("Checking for stored token...");
    let token = if let Some(stored_token) = TOKEN_STORAGE.get_token() {
        println!("Found stored token");
        stored_token
    } else {
        println!("No valid token found, obtaining new token...");
        let new_token = api::auth::get_initial_token().await.expect("Failed to get token");
        TOKEN_STORAGE.save_token(new_token.clone());
        println!("New token obtained and saved");
        new_token
    };

    let oauth_client = api::auth::OAuthClient::new(token);

    let symbol = "AAPL";
    let quotes = quote(symbol, &oauth_client).await.expect("Failed to get quotes");
    let chains = chains(symbol, &oauth_client).await.expect("Failed to get chains");
    test_utils::write_test_data(quotes, chains);
}



// fn _process(record: &Options, dividend: &Dividend) -> Option<Options> {
//     if record.premium() < 0.05 {
//         return None
//     }
//
//     let mut options = Options::new(record.clone());
//
//     options.return_after_1_div = Some(calculate_return_after_dividends(&record, dividend, 1));
//     options.return_after_2_div = Some(calculate_return_after_dividends(&record, dividend, 2));
//     options.return_after_3_div = Some(calculate_return_after_dividends(&record, dividend, 3));
//     options.return_after_4_div = Some(calculate_return_after_dividends(&record, dividend, 4));
//     options.return_after_5_div = Some(calculate_return_after_dividends(&record, dividend, 5));
//     options.return_after_6_div = Some(calculate_return_after_dividends(&record, dividend, 6));
//
//     Some(options)
// }


// fn update_returns() -> Result<(), Box<dyn std::error::Error>> {
//     let mut return_records: Vec<Options> = vec![];
//     let option_records: Vec<Options> = utils::get_options()?;
//
//
//     let dividend_map: HashMap<String, Dividend> = match utils::get_dividend_map() {
//         Ok(map) => {
//             map
//         },
//         Err(e) => {
//             println!("Something is up: {}", e);
//             return Err(e)
//         }
//     };
//
//     for record in &option_records {
//         let today = Utc::now().date_naive();
//
//         // we only want to process things in the next 20 or so months
//         let expiration_date = record.expiration_date();
//         if expiration_date > today + Duration::days(30*20) {
//             println!("The expiration date is too far out: {:?}", expiration_date);
//             continue
//         }
//
//         if record.last * 0.50 > record.strike_price {
//             println!("The strike price is too high compared to last: {:?} vs {:?}", record.strike_price, record.last);
//             continue
//         }
//
//         if let Some(dividend) = dividend_map.get(&record.symbol) {
//             if let Some(returns) = _process(&record, dividend) {
//                 // println!("{:?}", returns);
//                 return_records.push(returns)
//             }
//         } else {
//             // println!("No dividend found for symbol: {:?}", record.symbol);
//             continue
//         }
//     }
//     println!("{:?}", return_records.len());
//
//     let file = File::create("../data/rust_returns.csv")?;
//     let mut wtr = WriterBuilder::new().from_writer(file);
//     for record in &return_records {
//         wtr.serialize(record).map_err(|e| {
//             println!("Failed to serialize record: {}", e);
//             e
//         })?;
//
//     }
//     wtr.flush()?;
//
//     Ok(())
// }
