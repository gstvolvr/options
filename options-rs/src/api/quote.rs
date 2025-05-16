// Top-level struct to handle ticker symbol mapping
#[derive(serde::Deserialize, Debug)]
pub(crate) struct QuoteResponse {
    // Map of ticker symbols to quote data
    #[serde(flatten)]
    pub quotes: std::collections::HashMap<String, QuoteApiResponse>,
}

// struct based on the following json structure
#[derive(serde::Deserialize, Debug)]
// tell serde to expect cameCase keys in the JSON
#[serde(rename_all = "camelCase")]
pub(crate) struct QuoteApiResponse {
    pub asset_main_type: String,
    pub asset_sub_type: String,
    pub symbol: String,
    pub quote_type: String,
    pub realtime: bool,
    pub ssid: i64,
    pub reference: Reference,
    pub quote: Quote,
    pub extended: Extended,
    pub fundamental: Fundamental,
    pub regular: Regular,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Reference {
    pub cusip: String,
    pub description: String,
    pub exchange: String,
    pub exchange_name: String,
    pub is_hard_to_borrow: bool,
    pub is_shortable: bool,
    pub htb_quantity: i64,
    pub htb_rate: f64,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Quote {
    #[serde(rename = "52WeekHigh")]
    pub fifty_two_week_high: f64,
    #[serde(rename = "52WeekLow")]
    pub fifty_two_week_low: f64,
    #[serde(rename = "askMICId")]
    pub ask_mic_id: String,
    pub ask_price: f64,
    pub ask_size: i64,
    pub ask_time: i64,
    #[serde(rename = "bidMICId")]
    pub bid_mic_id: String,
    pub bid_price: f64,
    pub bid_size: i64,
    pub bid_time: i64,
    pub close_price: f64,
    pub high_price: f64,
    #[serde(rename = "lastMICId")]
    pub last_mic_id: String,
    pub last_price: f64,
    pub last_size: i64,
    pub low_price: f64,
    pub mark: f64,
    pub mark_change: f64,
    pub mark_percent_change: f64,
    pub net_change: f64,
    pub net_percent_change: f64,
    pub open_price: f64,
    pub post_market_change: f64,
    pub post_market_percent_change: f64,
    pub quote_time: i64,
    pub security_status: String,
    pub total_volume: i64,
    pub trade_time: i64,
    // pub volatility: f64,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Extended {
    pub ask_price: f64,
    pub ask_size: i64,
    pub bid_price: f64,
    pub bid_size: i64,
    pub last_price: f64,
    pub last_size: i64,
    pub mark: f64,
    pub quote_time: i64,
    pub total_volume: i64,
    pub trade_time: i64,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Fundamental {
    pub avg10_days_volume: i64,
    pub avg1_year_volume: i64,
    pub declaration_date: String,
    pub div_amount: f64,
    pub div_ex_date: String,
    pub div_freq: i64,
    pub div_pay_amount: f64,
    pub div_pay_date: String,
    pub div_yield: f64,
    pub eps: f64,
    pub fund_leverage_factor: f64,
    pub last_earnings_date: String,
    pub next_div_ex_date: String,
    pub next_div_pay_date: String,
    pub pe_ratio: f64,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Regular {
    pub regular_market_last_price: f64,
    pub regular_market_last_size: i64,
    pub regular_market_net_change: f64,
    pub regular_market_percent_change: f64,
    pub regular_market_trade_time: i64,
}
