use serde::Deserialize;
use chrono::{DateTime, Utc, NaiveDate, Duration};
use crate::util;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Options {
    //pub put_call: String,
    pub symbol: String,
    pub description: String,
    // pub exchange_name: String,
    pub bid: f64,
    pub ask: f64,
    pub last: f64,
    pub mark: f64,
    pub bid_size: String,
    pub ask_size: String,
    pub bid_ask_size: String,
    pub last_size: String,
    pub high_price: f64,
    pub low_price: f64,
    pub open_price: f64,
    pub close_price: f64,
    pub total_volume: String,
    pub trade_date: String,
    pub trade_time_in_long: String,
    pub quote_time_in_long: String,
    pub net_change: String,
    // pub volatility: String,
    // pub delta: f64,
    // pub gamma: f64,
    // pub theta: f64,
    // pub vega: f64,
    // pub rho: f64,
    // pub open_interest: String,
    // pub time_value: String,
    // pub theoretical_option_value: String,
    // pub theoretical_volatility: String,
    // pub option_deliverables_list: String,
    pub strike_price: f64,
    pub expiration_date: i64, // This is a unix timestamp
    pub days_to_expiration: String,
    // pub expiration_type: String,
    // pub last_trading_day: String,
    // pub multiplier: String,
    // pub settlement_type: String,
    // pub deliverable_note: String,
    // pub is_index_option: String,
    // pub percent_change: String,
    // pub mark_change: String,
    // pub mark_percent_change: String,
    // pub intrinsic_value: String,
    // pub in_the_money: String,
    // pub non_standard: String,
    // pub mini: String,
    // pub penny_pilot: String,
    // pub option_symbol: String,
    // pub expiry: String,
    // pub strike: String,
    // pub close: String,
    // pub quote_date: String
}

impl Options {
    /// calcualte the mid point
    pub fn mid(&self) -> f64 {
        (self.bid + self.ask) / 2.0
    }

    /// calculate the net position
    pub fn net(&self) -> f64 {
        self.last + self.mid()
    }

    /// calculate the premium
    pub fn premium(&self) -> f64 {
        self.strike_price - self.net()
    }

    /// calculate the insurance
    pub fn insurance(&self) -> f64 {
        (self.last - self.net()) / self.last
    }

    /// convert unix timestamp into a NaiveDate object
    pub fn expiration_date(&self) -> NaiveDate {
        util::unix_to_date(self.expiration_date)
    }
}
