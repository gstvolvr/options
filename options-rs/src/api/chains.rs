use std::collections::HashMap;
/// TODO:
/// - Finish writing up docstrings
/// - Determine at what level we want to do calculations
/// - Implement calculations at that level
/// -
/// Documentation: https://developer.schwab.com/products/trader-api--individual/details/documentation/Market%20Data%20Production

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChainsApiResponse {
    /// # Example: "AAPL"
    pub symbol: String,
    /// # Example: "SUCCESS"
    pub status: String,
    pub underlying: Underlying,
    /// # Example: "COVERED"
    pub strategy: String,
    /// # Example: 0.0
    pub interval: f64,
    /// # Example: false
    pub is_delayed: bool,
    /// # Example: false
    pub is_index: bool,
    /// # Example: 0.0
    pub days_to_expiration: f64,
    /// # Example: 0.0
    pub interest_rate: f64,
    /// # Example: 212.93
    pub underlying_price: f64,
    /// # Example: 0.0
    pub volatility: f64,
    /// # Example: 0.0
    pub dividend_yield: f64,
    /// # Example: 0
    pub number_of_contracts: i64,
    /// # Example: "EQUITY"
    pub asset_main_type: String,
    /// # Example: "COE"
    pub asset_sub_type: String,
    /// # Example: false
    pub is_chain_truncated: bool,
    /// # Example: []
    pub intervals: Vec<String>,
    pub monthly_strategy_list: Vec<MonthlyStrategy>,
    #[serde(default)]
    pub call_exp_date_map: HashMap<String, HashMap<String, Vec<OptionContract>>>,
    #[serde(default)]
    pub put_exp_date_map: HashMap<String, HashMap<String, Vec<OptionContract>>>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Underlying {
    /// # Example: 211.91
    pub ask: f64,
    /// Number of contracts for ask
    /// # Example: 3
    pub ask_size: i64,
    /// # Example: 211.85
    pub bid: f64,
    /// Number of contracts for bid
    /// # Example: 2
    pub bid_size: i64,
    /// # Example: 2.14
    pub change: f64,
    /// # Example: 212.93
    pub close: f64,
    /// # Example: false
    pub delayed: bool,
    /// # Example: "APPLE INC"
    pub description: String,
    /// # Example: "NASDAQ"
    pub exchange_name: String,
    /// # Example: 260.1
    pub fifty_two_week_high: f64,
    /// # Example: 169.21
    pub fifty_two_week_low: f64,
    /// # Example: 0.0
    pub high_price: f64,
    /// # Example: 212.93
    pub last: f64,
    /// # Example: 0.0
    pub low_price: f64,
    /// # Example: 211.91
    pub mark: f64,
    /// # Example: -1.02
    pub mark_change: f64,
    /// # Example: -0.48
    pub mark_percent_change: f64,
    /// # Example: 0.0
    pub open_price: f64,
    /// # Example: 1.01
    pub percent_change: f64,
    /// # Example: 1747220215334
    pub quote_time: i64,
    /// # Example: "AAPL"
    pub symbol: String,
    /// # Example: 173911.0
    pub total_volume: f64,
    /// # Example: 1747220214053.0
    pub trade_time: f64,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CallExpDateMap {
    pub additional_prop1: AdditionalProp,
    pub additional_prop2: AdditionalProp,
    pub additional_prop3: AdditionalProp,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PutExpDateMap {
    pub additional_prop1: AdditionalProp,
    pub additional_prop2: AdditionalProp,
    pub additional_prop3: AdditionalProp,
}


#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalProp {
    pub put_call: String,
    pub symbol: String,
    pub description: String,
    pub exchange_name: String,
    pub bid_price: f64,
    pub ask_price: f64,
    pub last_price: f64,
    pub mark_price: f64,
    pub bid_size: i64,
    pub ask_size: i64,
    pub last_size: i64,
    pub high_price: f64,
    pub low_price: f64,
    pub open_price: f64,
    pub close_price: f64,
    pub total_volume: i64,
    pub trade_date: i64,
    pub quote_time_in_long: i64,
    pub trade_time_in_long: i64,
    pub net_change: f64,
    pub volatility: f64,
    pub delta: f64,
    pub gamma: f64,
    pub theta: f64,
    pub vega: f64,
    pub rho: f64,
    pub time_value: f64,
    pub open_interest: i64,
    pub is_in_the_money: bool,
    pub theoretical_option_value: f64,
    pub theoretical_volatility: f64,
    pub is_mini: bool,
    pub is_non_standard: bool,
    pub option_deliverables_list: Vec<OptionDeliverables>,
    pub strike_price: f64,
    pub expiration_date: String,
    pub days_to_expiration: f64,
    pub expiration_type: String,
    pub last_trading_day: i64,
    pub multiplier: i64,
    pub settlement_type: String,
    pub deliverable_note: String,
    pub is_index_option: bool,
    pub percent_change: f64,
    pub mark_change: f64,
    pub mark_percent_change: f64,
    pub is_penny_pilot: bool,
    pub intrinsic_value: f64,
    pub option_root: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OptionDeliverables {
    pub symbol: String,
    pub asset_type: String,
    pub deliverable_units: String,
    pub currency_type: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MonthlyStrategy {
    /// # Example: "May"
    pub month: String,
    /// # Example: 2025
    pub year: i64,
    /// # Example: 16
    pub day: i64,
    /// # Example: 2
    pub days_to_exp: i64,
    /// # Example: 0
    pub secondary_year: i64,
    /// # Example: 0
    pub secondary_day: i64,
    /// # Example: 0
    pub secondary_days_to_exp: i64,
    /// # Example: "C"
    #[serde(rename = "type")]
    pub type_: String,
    /// # Example: " "
    pub secondary_type: String,
    /// # Example: false
    pub leap: bool,
    pub option_strategy_list: Vec<OptionStrategy>,
    /// # Example: false
    pub secondary_leap: bool,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OptionStrategy {
    pub primary_leg: PrimaryLeg,
    /// # Example: "200.0"
    pub strategy_strike: String,
    /// # Example: 198.5
    pub strategy_bid: f64,
    /// # Example: 198.91
    pub strategy_ask: f64,
    /// # Example: 199.75
    pub strategy_mark: f64,
    /// # Example: 1.956896685975224
    pub strategy_delta: f64,
    /// # Example: -0.011589677530975851
    pub strategy_gamma: f64,
    /// # Example: 0.10840225513919677
    pub strategy_theta: f64,
    /// # Example: -0.017925816454180676
    pub strategy_vega: f64,
    /// # Example: -0.016093830489012362
    pub strategy_rho: f64,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PrimaryLeg {
    /// Ticker symbol. Using the following format:
    /// {EQUITY}  {EXP_IN_YYMMDD}{PUT_CALL_IND}{STRIKE_PRICE}
    /// Represents:
    ///     1. Underlying equity
    ///     2. Options contract expiration date
    ///     3. Strike price
    /// # Example: "AAPL  250516C00200000"
    pub symbol: String,
    /// Put / Call Indicator
    /// # Example: "C" or "P"
    pub put_call_ind: String,
    /// Pretty version of `symbol`.
    /// # Example: "AAPL 05/16/2025 200.00 C"
    pub description: String,
    /// Current Bid Price in $
    /// # Example: 13.0
    pub bid: f64,
    /// Current Ask Price in $
    /// # Example: 13.55
    pub ask: f64,
    pub volume: i64,
    pub range: String,
    pub strike_price: f64,
    pub settlement_type: String,
    pub expiration_type: String,
    /// Price at which the last trade was matched
    pub last_price: f64,
    /// Mark Price
    pub mark: f64,
    /// Number of contracts for bid
    pub bid_size: i64,
    /// Number of contracts for ask
    pub ask_size: i64,
    /// Number of contracts traded with last trade
    pub last_size: i64,
    /// Day's high trade price
    pub high_price: f64,
    /// Day's low trade price
    pub low_price: f64,
    /// 52 Week High
    pub high52_week: f64,
    /// 52 Week Low 
    pub low52_week: f64,
    /// Day's Open Price Yes No According to industry standard, only regular session trades set the open
    /// If a stock does not trade during the regular session, then the open price is 0.
    /// In the pre-market session, open is blank because pre-market session trades do not set the open.
    /// Open is set to ZERO at 7:28 ET.
    pub open_price: f64,
    /// Previous day's closing price
    pub close_price: f64,
    /// Open Interest
    pub open_interest: i64,
    /// Current Last-Prev Close
    pub net_change: f64,
    /// Option Risk/Volatility Measurement/Implied
    pub volatility: f64,
    /// Greeks
    pub delta: f64,
    pub gamma: f64,
    pub theta: f64,
    pub vega: f64,
    pub rho: f64,
    /// Last trade time in milliseconds since Epoch
    pub trade_time_in_long: i64,
    /// Last quote time in milliseconds since Epoch
    pub quote_time_in_long: i64,
    /// Net Percentage Change
    pub percent_change: f64,
    /// The value an option would have if it were exercised today. Basically, the intrinsic value is the amount by which the strike price of an option is profitable or in-the-money as compared to the underlying stock's price in the market.
    pub intrinsic_value: f64,
    pub extrinsic_value: f64,
    pub total_volume: f64,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OptionContract {
    /// # Example: "C"
    pub put_call: String,
    /// # Example: "AAPL  250516C00200000"
    pub symbol: String,
    /// # Example: "AAPL 05/16/2025 200.00 C"
    pub description: String,
    /// # Example: "NASDAQ" 
    pub exchange_name: String,
    /// # Example: 13.0
    pub bid_price: f64,
    /// # Example: 13.35
    pub ask_price: f64,
    /// # Example: 13.21
    pub last_price: f64,
    /// # Example: 13.18
    pub mark_price: f64,
    /// # Example: 11
    pub bid_size: i64,
    /// # Example: 17
    pub ask_size: i64,
    /// # Example: 1
    pub last_size: i64,
    /// # Example: 0.0
    pub high_price: f64,
    /// # Example: 0.0
    pub low_price: f64,
    /// # Example: 0.0
    pub open_price: f64,
    /// # Example: 13.1395
    pub close_price: f64,
    /// # Example: 0
    pub total_volume: i64,
    /// # Example: 1747166396890
    pub trade_date: i64,
    /// # Example: 1747166400130
    pub quote_time_in_long: i64,
    /// # Example: 1747166396890
    pub trade_time_in_long: i64,
    /// # Example: 0.07
    pub net_change: f64,
    /// # Example: 40.2723985931935
    pub volatility: f64,
    /// # Example: 0.9568966859752239
    pub delta: f64,
    /// # Example: 0.011589677530975851
    pub gamma: f64,
    /// # Example: -0.10840225513919677
    pub theta: f64,
    /// # Example: 0.017925816454180676
    pub vega: f64,
    /// # Example: 0.016093830489012362
    pub rho: f64,
    /// # Example: 0.28
    pub time_value: f64,
    /// # Example: 16043
    pub open_interest: i64,
    /// # Example: true
    pub is_in_the_money: bool,
    /// # Example: 13.21
    pub theoretical_option_value: f64,
    /// # Example: 40.2723985931935
    pub theoretical_volatility: f64,
    /// # Example: false
    pub is_mini: bool,
    /// # Example: false
    pub is_non_standard: bool,
    pub option_deliverables_list: Vec<OptionDeliverables>,
    /// # Example: 200.0
    pub strike_price: f64,
    /// # Example: "2025-05-16"
    pub expiration_date: String,
    /// # Example: 2.0
    pub days_to_expiration: f64,
    /// # Example: "S"
    pub expiration_type: String,
    /// # Example: 1747166396890
    pub last_trading_day: i64,
    /// # Example: 100
    pub multiplier: i64,
    /// # Example: "P"
    pub settlement_type: String,
    /// # Example: ""
    pub deliverable_note: String,
    /// # Example: false
    pub is_index_option: bool,
    /// # Example: 0.53
    pub percent_change: f64,
    /// # Example: 0.08
    pub mark_change: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_chains_response_deserialization() {
        let test_data_path = Path::new("src/api/test_data/chains_test_data.json");
        let json_str = fs::read_to_string(test_data_path)
            .expect("Failed to read test data file");
        let json_data: serde_json::Value = serde_json::from_str(&json_str)
            .expect("Failed to parse JSON from test data file");

        let result: Result<ChainsApiResponse, _> = serde_json::from_value(json_data);
        assert!(result.is_ok());

        let chains = result.unwrap();
        assert_eq!(chains.symbol, "AAPL");
        assert_eq!(chains.status, "SUCCESS");
        assert_eq!(chains.strategy, "COVERED");
        assert_eq!(chains.underlying.ask, 211.91);
        assert_eq!(chains.underlying.bid, 211.85);
    }
}
