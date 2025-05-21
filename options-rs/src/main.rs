use csv::WriterBuilder;
use chrono::{Duration, Utc};
// use crate::models::dividend::Dividend;
// use crate::models::options::Options;
// use options_rs::utils::calculate_return_after_dividends;
use options_rs::test_utils;
use options_rs::api;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::option;
use std::string::ToString;
use options_rs::api::chains::ChainsApiResponse;
use options_rs::api::quote::QuoteApiResponse;
use options_rs::api::schwab::{chains, quote};
use lazy_static::lazy_static;
use options_rs::api::auth::OAuthClient;
use options_rs::api::token_storage::TOKEN_STORAGE;

static DATA_DIR_PATH: &str = "../data";
static QUOTES_FILENAME: &str = "schwab_quotes.json";
static CHAINS_FILENAME: &str = "schwab_chains.json";
static SYMBOLS_FILENAME: &str = "symbols_mini.csv"; // TODO: swap to `symbols.csv` when ready

lazy_static! {
    static ref QUOTES_DATA_PATH: String = format!("{}/{}", DATA_DIR_PATH, QUOTES_FILENAME);
    static ref CHAINS_DATA_PATH: String = format!("{}/{}", DATA_DIR_PATH, CHAINS_FILENAME);
    static ref SYMBOLS_DATA_PATH: String = format!("{}/{}", DATA_DIR_PATH, SYMBOLS_FILENAME);
}

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

    // generating test data
    // let symbol = "AAPL";
    // let quotes = quote(symbol, &oauth_client).await.expect("Failed to get quotes");
    // let chains = chains(symbol, &oauth_client).await.expect("Failed to get chains");
    // test_utils::write_test_data(quotes, chains);

    if let Err(e) = write_api_data_for_all_tickers(oauth_client).await {
        eprintln!("Error processing symbols file: {}", e);
    }
}

async fn write_api_data_for_all_tickers(oauth_client: OAuthClient) -> Result<(), Box<dyn std::error::Error>> {
    let file = OpenOptions::new()
        .read(true)
        .open(&*SYMBOLS_DATA_PATH)?;

    // reset data files
    create_api_data_files();
    let mut rdr = csv::Reader::from_reader(file);

    for result in rdr.records() {
        let record = result?;
        let symbol = record.get(0).unwrap();
        println!("Processing symbol: {:?}", symbol);
        let quotes = quote(symbol, &oauth_client).await.expect("Failed to get quotes");
        let chains = chains(symbol, &oauth_client).await.expect("Failed to get chains");
        append_api_data(quotes, chains)
    };

    Ok(())
}

fn create_api_data_files() -> () {
    OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&*QUOTES_DATA_PATH)
        .expect("Failed to open quotes file");

    OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&*CHAINS_DATA_PATH)
        .expect("Failed to open quotes file");
}

use std::io::Write;

fn append_api_data(quotes: QuoteApiResponse, chains: ChainsApiResponse) -> () {
    let mut quotes_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&*QUOTES_DATA_PATH)
        .expect("Failed to open quotes file");

    let mut chains_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&*CHAINS_DATA_PATH)
        .expect("Failed to open chains file");

    quotes_file
        .write_all(format!("{},\n", serde_json::to_string(&quotes).expect("Failed to serialize quotes")).as_bytes())
        .expect("Failed to write quotes");

    chains_file
        .write_all(format!("{},\n", serde_json::to_string(&chains).expect("Failed to serialize chains")).as_bytes())
        .expect("Failed to write chains");
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
