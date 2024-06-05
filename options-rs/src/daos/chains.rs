use std::fs::File;
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


fn update_eod_options() {
    println!("Updating EOD options prices...");
    // TODO: do some error checking here
    let dividends: Vec<Dividend> = util::get_dividends().unwrap();
    let file = File::create("../data/rust_options.csv")?;
    let mut wtr = WriterBuilder::new().from_writer(file);
    for dividend in dividends {
        println!("{:?}", dividend);
    }
}
