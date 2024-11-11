// struct based on the following json structure
#[derive(serde::Deserialize, Debug)]
pub(crate) struct QuoteApiResponse {
    pub asset_main_type: String,
    pub symbol: String,
    pub quote_type: String,
    pub realtime: bool,
    pub ssid: i64,
    pub reference: Reference,
    pub quote: Quote,
}

#[derive(serde::Deserialize, Debug)]
pub struct Reference {
    pub cusip: String,
    pub description: String,
    pub exchange: String,
    pub exchange_name: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct Quote {
    pub fifty_two_week_high: f64,
    pub fifty_two_week_low: f64,
    pub ask_mic_id: String,
    pub ask_price: f64,
    pub ask_size: i64,
    pub ask_time: i64,
    pub bid_mic_id: String,
    pub bid_price: f64,
    pub bid_size: i64,
    pub bid_time: i64,
    pub close_price: f64,
    pub high_price: f64,
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
    pub quote_time: i64,
    pub security_status: String,
    pub total_volume: i64,
    pub trade_time: i64,
    pub volatility: f64,
}
