use csv::{Writer, WriterBuilder};
use chrono::{Duration, Utc};
// use crate::models::dividend::Dividend;
// use crate::models::options::Options;
// use options_rs::utils::calculate_return_after_dividends;
use options_rs::test_utils;
use options_rs::api;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::{io, option};
use std::io::{BufRead};
use std::string::ToString;
use options_rs::api::chains::ChainsApiResponse;
use options_rs::api::quote::QuoteApiResponse;
use options_rs::api::schwab::{chains, quote};
use lazy_static::lazy_static;
use options_rs::api::auth::OAuthClient;
use options_rs::api::token_storage::TOKEN_STORAGE;
use serde_json::Value;


static DATA_DIR_PATH: &str = "../data";
static QUOTES_FILENAME: &str = "schwab_quotes.jsonl";
static CHAINS_FILENAME: &str = "schwab_chains.jsonl";
static RETURNS_FILENAME: &str = "schwab_returns.csv";
static SYMBOLS_FILENAME: &str = "symbols.csv"; // TODO: swap to `symbols.csv` when ready

lazy_static! {
    static ref QUOTES_DATA_PATH: String = format!("{}/{}", DATA_DIR_PATH, QUOTES_FILENAME);
    static ref CHAINS_DATA_PATH: String = format!("{}/{}", DATA_DIR_PATH, CHAINS_FILENAME);
    static ref SYMBOLS_DATA_PATH: String = format!("{}/{}", DATA_DIR_PATH, SYMBOLS_FILENAME);
    static ref RETURNS_DATA_PATH: String = format!("{}/{}", DATA_DIR_PATH, RETURNS_FILENAME);
}

#[tokio::main]
async fn main() {
    println!("Checking for stored token...");
    // let token = if let Some(stored_token) = TOKEN_STORAGE.get_token() {
    //     println!("Found stored token");
    //     stored_token
    // } else {
    //     println!("No valid token found, obtaining new token...");
    //     let new_token = api::auth::get_initial_token().await.expect("Failed to get token");
    //     TOKEN_STORAGE.save_token(new_token.clone());
    //     println!("New token obtained and saved");
    //     new_token
    // };
    //
    // let oauth_client = api::auth::OAuthClient::new(token);

    // generating test data
    // let symbol = "AAPL";
    // let quotes = quote(symbol, &oauth_client).await.expect("Failed to get quotes");
    // let chains = chains(symbol, &oauth_client).await.expect("Failed to get chains");
    // test_utils::write_test_data(quotes, chains);

    // if let Err(e) = write_api_data_for_all_tickers(oauth_client).await {
    //     eprintln!("Error processing symbols file: {}", e);
    // }
    calculate_returns().await.expect("Failed to calculate returns");
}

fn read_json_lines<T: DeserializeOwned>(filepath: &str) -> io::Result<Vec<T>> {
    let file = File::open(filepath)?;
    let reader = io::BufReader::new(file);
    let mut objects = Vec::new();

    for line in reader.lines() {
        let mut line = line?;
        if line.ends_with(',') {
            line.pop();
        }
        if !line.trim().is_empty() {
            let value: T = serde_json::from_str(&line)?;
            objects.push(value);
        }
    }

    Ok(objects)
}

async fn calculate_returns() -> Result<(), Box<dyn std::error::Error>> {
    let quotes: Vec<QuoteApiResponse> = read_json_lines(&*QUOTES_DATA_PATH)?;
    let chains: Vec<ChainsApiResponse> = read_json_lines(&*CHAINS_DATA_PATH)?;

    let file = File::create(&*RETURNS_DATA_PATH)?;
    let mut wtr = WriterBuilder::new().from_writer(file);

    // Write header row
    wtr.write_record(&[
        "symbol", "company_name", "industry", "last", "net", "strike_price",
        "expiration_date", "insurance", "premium", "dividend_quarterly_amount", "dividend_ex_date",
        "return_after_1_div", "return_after_2_div", "return_after_3_div", "return_after_4_div", "return_after_5_div",
        "return_after_last_div", "bid", "mid", "ask", "previous_date"
    ])?;

    for (quote, chain) in quotes.iter().zip(chains.iter()) {
        for (expiration_date, strikes) in &chain.call_exp_date_map {
            // println!("Processing expiration date: {:?}", expiration_date);
            for (strike_price, contracts) in strikes {
                // println!("Processing strike price: {:?}", strike_price);
                for contract in contracts {
                    if contract.should_ignore(quote.quote.last_price).unwrap_or(true) {
                        continue;
                    }

                    let mid = contract.mid().unwrap();
                    let net = contract.buy_write_cost_basis(quote.quote.last_price).unwrap();
                    let insurance = contract.buy_write_insurance(quote.quote.last_price).unwrap();
                    let premium = contract.buy_write_premium(quote.quote.last_price).unwrap();
                    let returns = &(1..=5)
                            .map(|n| contract.calculate_return_after_dividend(
                                quote.quote.last_price,
                                quote.fundamental.clone(),
                                n,
                                None,
                            ).to_string())
                            .collect::<Vec<String>>();

                    let last_return = returns.iter()
                        .rev()
                        .find(|s| !s.is_empty() && s != &"0")
                        .unwrap_or(&"0".to_string())
                        .to_string();

                    wtr.write_record(&[
                        &quote.symbol,
                        "",
                        "",
                        &quote.quote.last_price.to_string(),
                        &net.to_string(),
                        &strike_price.to_string(),
                        expiration_date,
                        &insurance.to_string(),
                        &premium.to_string(),
                        &quote.fundamental.div_amount.to_string(),
                        &quote.fundamental.div_ex_date,
                        &returns[0],
                        &returns[1],
                        &returns[2],
                        &returns[3],
                        &returns[4],
                        &last_return,
                        &contract.bid_price.unwrap_or_default().to_string(),
                        &mid.to_string(),
                        &contract.ask_price.unwrap_or_default().to_string(),
                        ""
                    ])?;
                }
            }
        }
    }

    wtr.flush()?;
    Ok(())
}

async fn write_api_data_for_all_tickers(oauth_client: OAuthClient) -> Result<(), Box<dyn std::error::Error>> {
    let file = OpenOptions::new()
        .read(true)
        .open(&*SYMBOLS_DATA_PATH)?;

    // reset data files
    create_api_data_files();
    let mut rdr = csv::Reader::from_reader(file);

    for result in rdr.records() {
        let record = match result {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Error reading record: {}", e);
                continue;
            }
        };
        let symbol = match record.get(0) {
            Some(s) => s,
            None => {
                eprintln!("No symbol found in record");
                continue;
            }
        };
        println!("Processing symbol: {:?}", symbol);

        let quotes = match quote(symbol, &oauth_client).await {
            Ok(q) => q,
            Err(e) => {
                eprintln!("Failed to get quotes for {}: {}", symbol, e);
                continue;
            }
        };

        let chains = match chains(symbol, &oauth_client).await {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Failed to get chains for {}: {}", symbol, e);
                continue;
            }
        };

        append_api_data(quotes, chains);
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
use serde::de::DeserializeOwned;

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
