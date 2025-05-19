use crate::models;
use chrono::{DateTime, NaiveDate};
use csv::ReaderBuilder;
// use models::dividend::Dividend;
// use models::options::Options;
use phf::phf_map;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;


static FREQUENCY_MAPPING: phf::Map<&str, i64> = phf_map! {
    "quarterly" => 3,
    "semi-annual" => 6,
    "monthly" => 1,
    "bimonthly" => 2
};

static MONTHS_IN_QUARTER: i64 = 3;

// pub fn calculate_return_after_dividends(record: &Options, dividend: &Dividend, n_dividends: i64) -> f64 {
//     let months_in = FREQUENCY_MAPPING.get(&dividend.dividend_frequency).unwrap();
//     // println!("{:?}", months_in);
//     // TODO: review this
//     //let dividend_ex_date: NaiveDate = DateTime::from_timestamp(dividend.dividend_ex_date, 0).unwrap().date_naive();
//     // println!("{:?}", months_in);
//     //let next_dividend_date: NaiveDate = dividend_ex_date + Duration::days((30 * months_in * (n_dividends)).into());
//     // println!("{:?}", record);
//     // figure out how to check for dates
//     0.0
// }


/// generic function to read CSV file
pub fn read_csv<T: for<'de> Deserialize<'de>>(file_path: &str) -> Result<Vec<T>, Box<dyn Error>> {
    let file = File::open(file_path).map_err(|e| {
        println!("Failed to open file: {}", e);
        e
    })?;
    println!("{:?}", file_path);
    let buf_reader = BufReader::new(file);
    let mut rdr = ReaderBuilder::new().from_reader(buf_reader);
    let mut records = vec![];

    for result in rdr.deserialize() {
        match result {
            Ok(record) => records.push(record),
            Err(e) => {
                println!("Failed to deserialize record: {}", e);
                return Err(Box::new(e));
            }
        }
    }
    Ok(records)
}

/// generic function to write CSV file
// fn write_csv<T: Serialize>(path: &str, records: &[T]) -> Result<(), Box<dyn Error>> {
//     let file = File::create(path)?;
//     let mut wtr = WriterBuilder::new().from_writer(file);
//     if !records.is_empty() {
//         wtr.write_record(&records[0].serialize_to_vec())?;
//         for record in records {
//             wtr.serialize(record)?;
//         }
//     }
//     wtr.flush()?;
//     Ok(())
// }

/// load local dividends data into a HashMap keyed by the symbol
// pub fn get_dividend_map() -> Result<HashMap<String, Dividend>, Box<dyn Error>> {
//     let dividends: Vec<Dividend> = read_csv("../data/dividends.csv")?;
//     // println!("{:?}", dividends);
//     // println!("{:?}", "here");
//     let dividends: HashMap<String, Dividend> = dividends
//         .into_iter()
//         .map(|d| (d.dividend_symbol.clone(), d))
//         .collect();
//     Ok(dividends)
// }
//
// /// load local options-py data into a Vector
// pub fn get_options() -> Result<Vec<Options>, Box<dyn Error>> {
//     read_csv("../data/options-py.csv")
// }
//
// /// load local dividends data into a Vector
// pub fn get_dividends() -> Result<Vec<Options>, Box<dyn Error>> {
//     read_csv("../data/dividends.csv")
// }

/// convert a unix timestamp into a chrono::NaiveDate
pub fn unix_to_date(timestamp: i64) -> NaiveDate {
    let timestamp: i64 = timestamp / 1000;
    // TODO: add proper handling in case this isn't a valid value
    let date: NaiveDate = DateTime::from_timestamp(timestamp, 0).unwrap().date_naive();
    date
}

/// Convert common date patterns used by the API to a NaviveDate

pub fn parse_date(date_str: &str) -> Result<NaiveDate, Box<dyn Error>> {
    if date_str.contains('T') {
        // Parse ISO 8601 format (with time component)
        match DateTime::parse_from_rfc3339(date_str) {
            Ok(datetime) => Ok(datetime.date_naive()),
            Err(e) => Err(Box::new(e))
        }
    } else {
        // Fallback to simple date format
        match NaiveDate::from_str(date_str) {
            Ok(date) => Ok(date),
            Err(e) => Err(Box::new(e))
        }
    }
}
