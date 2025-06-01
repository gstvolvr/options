use csv::{Writer, WriterBuilder};
use chrono::{Duration, Utc, NaiveDateTime, NaiveDate};
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
use tokio::sync::Mutex;
use tokio::task;
use tokio::time::{sleep, Duration as TokioDuration};
use std::sync::Arc;
use serde::de::DeserializeOwned;
use options_rs::config::{QUOTES_DATA_PATH, CHAINS_DATA_PATH, COMPANIES_DATA_PATH, RETURNS_DATA_PATH, SYMBOLS_DATA_PATH};



#[tokio::main]
async fn main() {
    env_logger::Builder::from_default_env().format_timestamp(None).init();
    log::debug!("Checking for stored token...");
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

    //// generating test data
    // let symbol = "AAPL";
    // let quotes = quote(symbol, &oauth_client).await.expect("Failed to get quotes");
    // let chains = chains(symbol, &oauth_client).await.expect("Failed to get chains");
    // test_utils::write_test_data(quotes, chains);

    if let Err(e) = write_api_data_for_all_tickers(oauth_client).await {
        eprintln!("Error processing symbols file: {}", e);
    }
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

async fn calculate_returns() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let quotes: Vec<QuoteApiResponse> = read_json_lines(&*QUOTES_DATA_PATH)?;
    let chains: Vec<ChainsApiResponse> = read_json_lines(&*CHAINS_DATA_PATH)?;

    // Load companies data
    let companies_file = File::open(&*COMPANIES_DATA_PATH)?;
    let mut companies_reader = csv::Reader::from_reader(companies_file);
    let mut companies: HashMap<String, (String, String)> = HashMap::new();

    for result in companies_reader.records() {
        let record = result?;
        if let (Some(symbol), Some(company_name), Some(industry)) = (record.get(0), record.get(1), record.get(3)) {
            companies.insert(symbol.to_string(), (company_name.to_string(), industry.to_string()));
        }
    }

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
        for (composite_expiration_date, strikes) in &chain.call_exp_date_map {
            // This comes in the format of `2025-07-18:56`
            let expiration_date = &composite_expiration_date[..10];
            // println!("Processing expiration date: {:?}", expiration_date);
            for (strike_price, contracts) in strikes {
                // println!("Processing strike price: {:?}", strike_price);
                for contract in contracts {
                    // TODO: remove these two cases
                    // if quote.symbol != "NEE" {
                    //     continue
                    // }
                    // if contract.strike_price != 55.0 || expiration_date != "2026-01-16" {
                    //     continue
                    // }
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
                                Some(NaiveDate::from_ymd(2025, 5, 26)),
                            ).to_string())
                            .collect::<Vec<String>>();

                    let last_return = returns.iter()
                        .rev()
                        .find(|s| !s.is_empty() && s != &"0")
                        .unwrap_or(&"0".to_string())
                        .to_string();

                    // Look up company_name and industry from companies HashMap
                    let (company_name, industry) = companies.get(&quote.symbol)
                        .map(|(name, ind)| (name.as_str(), ind.as_str()))
                        .unwrap_or(("", ""));

                    wtr.write_record(&[
                        &quote.symbol,
                        company_name,
                        industry,
                        &quote.quote.last_price.to_string(),
                        &net.to_string(),
                        &strike_price.to_string(),
                        &expiration_date.clone(),
                        &insurance.to_string(),
                        &premium.to_string(),
                        &quote.fundamental.div_amount.to_string(),
                        &parse_date(&quote.fundamental.div_ex_date).map(|dt| dt.format("%Y-%m-%d").to_string()).unwrap_or_else(|_| quote.fundamental.div_ex_date.clone()),
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

async fn write_api_data_for_all_tickers(oauth_client: OAuthClient) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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
use options_rs::utils::parse_date;

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


