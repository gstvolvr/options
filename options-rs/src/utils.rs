use chrono::{Datelike, DateTime, Duration, NaiveDate, Utc, Months};
use csv::ReaderBuilder;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;


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
            Ok(datetime) => Ok(datetime.with_timezone(&Utc).date_naive()),
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

pub fn round_to_decimals(value: f64, decimals: Option<f64>) -> f64 {
    let decimals = decimals.unwrap_or(2.0);
    (value * 10.0_f64.powf(decimals)).round() / (10.0_f64.powf(decimals))
}

/// If a date falls on a weekend, pick the prior Friday
pub fn get_previous_weekday(date: NaiveDate) -> NaiveDate {
    let mut date = date;
    let weekday = date.weekday().num_days_from_monday();
    if weekday == 6 { // Sunday
        date = date - Duration::days(2)
    } else if weekday == 5 { // Saturday
        date = date - Duration::days(1)
    }
    date
}

/// Projects a given date forward by a quarter (3 months) or whatever the expected dividend frequency is.
/// Then adjusts it to the previous Friday if the projected date falls on a Saturday or Sunday.
/// This accounts for a common financial calendar rule for ex-dividend dates.
///
/// TODO: look at historical patterns / ticker to determine most likely dividend date
pub fn estimate_next_dividend_date(date: NaiveDate, n_dividends: i64, div_freq: i64) -> NaiveDate {
    let months_between_dividends = 12 / div_freq;
    let months_until_next_dividend_capture = months_between_dividends * n_dividends;
    date.checked_add_months(Months::new(months_until_next_dividend_capture as u32)).unwrap()
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    #[test]
    fn round_decimals() {
        assert_eq!(
            round_to_decimals(1.23456789, Some(2.0)),
            1.23
        );

        assert_eq!(
            round_to_decimals(1.23456789, Some(3.0)),
            1.235
        );
    }
}
