use csv::WriterBuilder;
use options_rs::test_utils;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufRead};
use std::string::ToString;
use options_rs::api::chains::ChainsApiResponse;
use options_rs::api::quote::QuoteApiResponse;
use options_rs::api::schwab::{chains, quote};
use options_rs::api::auth::{authenticate, OAuthClient};
use options_rs::returns::Returns;
use serde_json::{json, Map, Value};
use serde::de::DeserializeOwned;
use options_rs::config::{QUOTES_DATA_PATH, CHAINS_DATA_PATH, COMPANIES_DATA_PATH, RETURNS_DATA_PATH, SYMBOLS_DATA_PATH, RETURNS_JSON_DATA_PATH};



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_default_env().format_timestamp(None).init();
    debug!("Checking for stored token...");

    let oauth_client = authenticate().await?;

    if let Err(e) = write_api_data_for_all_tickers(oauth_client).await {
        eprintln!("Error processing symbols file: {}", e);
    }
    if let Err(e) = calculate_returns().await {
        eprintln!("Error calculating returns: {}", e);
        std::process::exit(1);
    }

    Ok(())
}


async fn generate_test_data(oauth_client: OAuthClient) {
    let symbol = "AAPL";
    let quotes = quote(symbol, &oauth_client).await.expect("Failed to get quotes");
    let chains = chains(symbol, &oauth_client).await.expect("Failed to get chains");
    test_utils::write_test_data(quotes, chains);
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

/// Generate schwab_returns.csv and schwab_returns.json
/// The CSV file is used to load data into a Google Sheet
/// The JSON file is used to load data into Cloud Firestore
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

    // CSV writer
    let file = File::create(&*RETURNS_DATA_PATH)?;
    let mut wtr = WriterBuilder::new().from_writer(file);
    // JSON writer
    let json_file = File::create(&*RETURNS_JSON_DATA_PATH)?;
    let _json_wtr = serde_json::Serializer::new(json_file);
    let project_id: &str = "options";
    create_returns_data_files();

    // Write header row
    wtr.write_record(&[
        "symbol", "company_name", "industry", "last", "net", "strike_price",
        "expiration_date", "insurance", "premium", "dividend_quarterly_amount", "dividend_ex_date",
        "return_after_0_div", "return_after_1_div", "return_after_2_div", "return_after_3_div", "return_after_4_div", "return_after_5_div",
        "return_after_last_div", "bid", "mid", "ask", "previous_date"
    ])?;

    for (quote, chain) in quotes.iter().zip(chains.iter()) {
        for (composite_expiration_date, strikes) in &chain.call_exp_date_map {
            // This comes in the format of `2025-07-18:56`
            let expiration_date = &composite_expiration_date[..10];
            debug!("Processing expiration date: {:?}", expiration_date);
            for (strike_price, contracts) in strikes {
                debug!("Processing strike price: {:?}", strike_price);
                for contract in contracts {
                    if contract.should_ignore(quote.quote.last_price).unwrap_or(true) {
                        continue;
                    }

                    let mid = match contract.mid() {
                        Some(m) => m,
                        None => {
                            eprintln!("Unable to calculate mid price for {}", quote.symbol);
                            continue;
                        }
                    };
                    let cost_basis = match contract.buy_write_cost_basis(quote.quote.last_price) {
                        Some(cb) => cb,
                        None => {
                            eprintln!("Unable to calculate cost basis for {}", quote.symbol);
                            continue;
                        }
                    };
                    let insurance = match contract.buy_write_insurance(quote.quote.last_price) {
                        Some(ins) => ins,
                        None => {
                            eprintln!("Unable to calculate insurance for {}", quote.symbol);
                            continue;
                        }
                    };
                    let premium = match contract.buy_write_premium(quote.quote.last_price) {
                        Some(prem) => prem,
                        None => {
                            eprintln!("Unable to calculate premium for {}", quote.symbol);
                            continue;
                        }
                    };
                    let returns = &(0..=5)
                            .map(|n| contract.calculate_return_after_dividend(
                                quote.quote.last_price,
                                quote.fundamental.clone(),
                                n,
                                None,
                            ))
                            .collect::<Vec<Option<f64>>>();

                    let last_return = returns.iter()
                        .rev()
                        .find(|opt| opt.is_some() && opt.unwrap() != 0.0)
                        .and_then(|opt| *opt);

                    // Look up company_name and industry from companies HashMap
                    let (company_name, industry) = companies.get(&quote.symbol)
                        .map(|(name, ind)| (name.as_str(), ind.as_str()))
                        .unwrap_or(("", ""));

                    let returns = Returns {
                        symbol: quote.symbol.clone(),
                        company_name: company_name.to_string(),
                        industry: industry.to_string(),
                        last: quote.quote.last_price,
                        net: cost_basis,
                        strike_price: strike_price.clone(),
                        expiration_date: expiration_date.clone().to_string(),
                        insurance: insurance,
                        premium: premium,
                        dividend_quarterly_amount: quote.fundamental.div_amount,
                        dividend_ex_date: quote.fundamental.div_ex_date.as_ref()
                            .map(|date_str| parse_date(date_str).map(|dt| dt.format("%Y-%m-%d").to_string()).unwrap_or_else(|_| date_str.clone()))
                            .unwrap_or_else(|| "N/A".to_string()),
                        return_after_0_div: returns[0],
                        return_after_1_div: returns[1],
                        return_after_2_div: returns[2],
                        return_after_3_div: returns[3],
                        return_after_4_div: returns[4],
                        return_after_5_div: returns[5],
                        return_after_last_div: last_return,
                        bid: contract.bid_price.unwrap_or_default(),
                        mid: mid,
                        ask: contract.ask_price.unwrap_or_default(),
                        previous_date: "".to_string(),
                    };

                    // Write to CSV
                    wtr.write_record(&returns.to_csv_record())?;

                    // Append to already initialized JSON file
                    let doc = returns.to_firestore_document(project_id);
                    if vec!["AAPL", "NVDA"].contains(&returns.symbol.as_str()) {
                        append_returns_data(doc);
                    }
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
        info!("Processing symbol: {:?}", symbol);

        let quotes_data = match quote(symbol, &oauth_client).await {
            Ok(q) => q,
            Err(e) => {
                eprintln!("Failed to get quotes for {}: {}", symbol, e);
                continue;
            }
        };

        let chains_data = match chains(symbol, &oauth_client).await {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Failed to get chains for {}: {}", symbol, e);
                continue;
            }
        };

        append_api_data(quotes_data, chains_data);
    };

    Ok(())
}

fn create_api_data_files() -> () {
    if let Err(e) = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&*QUOTES_DATA_PATH) {
        eprintln!("Failed to create quotes file: {}", e);
        return;
    }

    if let Err(e) = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&*CHAINS_DATA_PATH) {
        eprintln!("Failed to create chains file: {}", e);
    }
}

fn create_returns_data_files() -> () {
    if let Err(e) = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&*RETURNS_JSON_DATA_PATH) {
        eprintln!("Failed to create returns file: {}", e);
    }
}

use std::io::Write;
use log::{debug, info};
use serde::{Serialize, Deserialize};
use options_rs::utils::parse_date;

fn append_returns_data(returns: Value) -> () {
    let mut file = match OpenOptions::new()
        .create(true)
        .append(true)
        .open(&*RETURNS_JSON_DATA_PATH) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open returns file: {}", e);
            return;
        }
    };
    let serialized = match serde_json::to_string(&returns) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to serialize returns: {}", e);
            return;
        }
    };
    if let Err(e) = file.write_all(format!("{}\n", serialized).as_bytes()) {
        eprintln!("Failed to write returns: {}", e);
    }
}

fn append_api_data(quotes: QuoteApiResponse, chains: ChainsApiResponse) -> () {
    let mut quotes_file = match OpenOptions::new()
        .create(true)
        .append(true)
        .open(&*QUOTES_DATA_PATH) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open quotes file: {}", e);
            return;
        }
    };

    let mut chains_file = match OpenOptions::new()
        .create(true)
        .append(true)
        .open(&*CHAINS_DATA_PATH) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open chains file: {}", e);
            return;
        }
    };

    let quotes_serialized = match serde_json::to_string(&quotes) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to serialize quotes: {}", e);
            return;
        }
    };

    let chains_serialized = match serde_json::to_string(&chains) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to serialize chains: {}", e);
            return;
        }
    };

    if let Err(e) = quotes_file.write_all(format!("{},\n", quotes_serialized).as_bytes()) {
        eprintln!("Failed to write quotes: {}", e);
    }

    if let Err(e) = chains_file.write_all(format!("{},\n", chains_serialized).as_bytes()) {
        eprintln!("Failed to write chains: {}", e);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    fn test_returns_to_csv_record() {
        let returns = Returns {
            symbol: "AAPL".to_string(),
            company_name: "Apple Inc.".to_string(),
            industry: "Technology".to_string(),
            last: 150.0,
            net: 140.0,
            strike_price: "160".to_string(),
            expiration_date: "2025-12-19".to_string(),
            insurance: 0.15,
            premium: 10.0,
            dividend_quarterly_amount: 0.25,
            dividend_ex_date: "2025-02-15".to_string(),
            return_after_1_div: 0.05,
            return_after_2_div: 0.10,
            return_after_3_div: 0.15,
            return_after_4_div: 0.20,
            return_after_5_div: 0.25,
            return_after_last_div: 0.20,
            bid: 9.5,
            mid: 10.0,
            ask: 10.5,
            previous_date: "2025-01-29".to_string(),
        };

        let csv_record = returns.to_csv_record();
        assert_eq!(csv_record[0], "AAPL");
        assert_eq!(csv_record[1], "Apple Inc.");
        assert_eq!(csv_record[3], "150");
        assert_eq!(csv_record.len(), 21);
    }

    #[test]
    fn test_returns_to_firestore_document() {
        let returns = Returns {
            symbol: "AAPL".to_string(),
            company_name: "Apple Inc.".to_string(),
            industry: "Technology".to_string(),
            last: 150.0,
            net: 140.0,
            strike_price: "160".to_string(),
            expiration_date: "2025-12-19".to_string(),
            insurance: 0.15,
            premium: 10.0,
            dividend_quarterly_amount: 0.25,
            dividend_ex_date: "2025-02-15".to_string(),
            return_after_1_div: 0.05,
            return_after_2_div: 0.10,
            return_after_3_div: 0.15,
            return_after_4_div: 0.20,
            return_after_5_div: 0.25,
            return_after_last_div: 0.20,
            bid: 9.5,
            mid: 10.0,
            ask: 10.5,
            previous_date: "2025-01-29".to_string(),
        };

        let doc = returns.to_firestore_document("test-project");
        assert!(doc["name"].as_str().unwrap().contains("AAPL_2025-12-19_160"));
        assert_eq!(doc["fields"]["symbol"]["stringValue"], "AAPL");
        assert_eq!(doc["fields"]["last"]["doubleValue"], 150.0);
    }

    #[test]
    fn test_read_json_lines_empty_file() {
        let temp_file = NamedTempFile::new().unwrap();
        let result: Result<Vec<serde_json::Value>, _> = read_json_lines(temp_file.path().to_str().unwrap());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[test]
    fn test_read_json_lines_valid_data() {
        let temp_file = NamedTempFile::new().unwrap();
        fs::write(temp_file.path(), "{\"test\": \"value1\"}
{\"test\": \"value2\"}").unwrap();

        let result: Result<Vec<serde_json::Value>, _> = read_json_lines(temp_file.path().to_str().unwrap());
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data.len(), 2);
        assert_eq!(data[0]["test"], "value1");
        assert_eq!(data[1]["test"], "value2");
    }
}
