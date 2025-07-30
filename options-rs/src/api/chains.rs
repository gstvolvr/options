#![allow(warnings)]

use std::cmp::max;
use std::collections::HashMap;
use chrono::{Months, NaiveDate};
use std::str::FromStr;
use crate::api::quote::Fundamental;
use crate::utils::{get_previous_weekday, parse_date, round_to_decimals};
use log::debug;

/// TODO:
/// - Finish writing up docstrings
/// - Calculate returns after `x` dividends
/// - Write output to `returns.csv` using the existin schema
/// Documentation: https://developer.schwab.com/products/trader-api--individual/details/documentation/Market%20Data%20Production

#[derive(serde::Serialize, serde::Deserialize, Debug)]
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
    // pub intervals: Vec<String>,
    // pub monthly_strategy_list: Vec<MonthlyStrategy>,
    #[serde(default)]
    pub call_exp_date_map: HashMap<String, HashMap<String, Vec<OptionContract>>>,
    #[serde(default)]
    pub put_exp_date_map: HashMap<String, HashMap<String, Vec<OptionContract>>>,
}


#[derive(serde::Serialize, serde::Deserialize, Debug)]
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

impl Underlying {
    pub fn mid(&self) -> f64 {
        (self.ask + self.bid) / 2.0
    }
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
    pub multiplier: f64,
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

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OptionDeliverables {
    pub symbol: String,
    pub asset_type: String,
    pub deliverable_units: f64,
    // sometimes not in the response
    pub currency_type: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
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

#[derive(serde::Serialize, serde::Deserialize, Debug)]
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

#[derive(serde::Serialize, serde::Deserialize, Debug)]
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
    /// # Example: ITM
    /// ITM: underlying asset price is above the strike price
    /// OTM: underlying asset price is below the strike price
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

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// When the markets are closed Schwab does not provide the latest `ask`, `bid` prices
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
    pub bid_price: Option<f64>,
    /// # Example: 13.35
    pub ask_price: Option<f64>,
    /// # Example: 13.21
    pub last_price: Option<f64>,
    /// # Example: 13.18
    pub mark_price: Option<f64>,
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
    pub close_price: Option<f64>,
    /// # Example: 0
    pub total_volume: i64,
    /// # Example: 1747166396890
    pub trade_date: Option<i64>,
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
    pub is_in_the_money: Option<bool>,
    /// # Example: 13.21
    pub theoretical_option_value: f64,
    /// # Example: 40.2723985931935
    pub theoretical_volatility: f64,
    /// # Example: false
    pub is_mini: Option<bool>,
    /// # Example: false
    pub is_non_standard: Option<bool>,
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
    /// # Example: 100.0
    pub multiplier: f64,
    /// # Example: "P"
    pub settlement_type: String,
    /// # Example: ""
    pub deliverable_note: String,
    /// # Example: false
    pub is_index_option: Option<bool>,
    /// # Example: 0.53
    pub percent_change: f64,
    /// # Example: 0.08
    pub mark_change: f64,
}

impl OptionContract {
    /// If the bid or ask prices are not defined, we fall back to the close price
    pub fn mid(&self) -> Option<f64> {
        match (self.bid_price, self.ask_price, self.close_price) {
            (Some(bid), Some(ask), _) => Some((ask + bid) / 2.0),
            (_, _, Some(last_price)) => Some(last_price),
            (_, _, _) => None
        }
    }

    /// Cost basis for a buy write trade given the current bid / ask spread
    pub fn buy_write_cost_basis(&self, underlying_equity_price: f64) -> Option<f64> {
        self.mid().map(|mid| underlying_equity_price - mid)
    }

    fn intrinsic_value(&self) -> f64 {
        0.0
    }

    fn premium(&self) -> f64 {
        self.time_value + self.intrinsic_value()
        // self.buy_write_cost_basis(underlying_equity_price).map(|net| self.strike_price - net)
    }
    /// TODO: add explanation about options-py premiums
    /// Assuming we're talking about call options
    /// The `premium` is broken out into 2 components
    /// 1. Intrinsic value: the actual value of the option if exercise immediately
    ///     max(0, Underlying Equity Price - Strike Price)
    /// 2. Extrinsic value (time value): the additional premium above the intrinsic value
    pub fn buy_write_premium(&self, underlying_equity_price: f64) -> Option<f64> {
        self.buy_write_cost_basis(underlying_equity_price).map(|net| self.strike_price - net)
    }

    /// Downside protection you have on the position
    pub fn buy_write_insurance(&self, underlying_equity_price: f64) -> Option<f64> {
        self.buy_write_cost_basis(underlying_equity_price).map(|net| (underlying_equity_price - net) / underlying_equity_price)
    }

    /// Sometimes we get invalid values from the API
    pub fn should_ignore(&self, underlying_equity_price: f64) -> Result<bool, String> {
        Ok(
            self.buy_write_cost_basis(underlying_equity_price).is_none() ||
            underlying_equity_price * 0.50 > self.strike_price ||
            // Don't bother with positions that expire within a quarter
            self.days_to_expiration <= 90.0 ||
            // Sometimes a premium is unrealistic
            self.buy_write_premium(underlying_equity_price).unwrap_or(0.0) < 0.05
        )
    }

    /// Calculate the annualized return of a buy write trade after capturing a certain number of dividend payments
    /// TODO: add captured dividend amount and captured dividend dates to the logs
    pub fn calculate_return_after_dividend(&self, underlying_equity_price: f64, fundamental: Fundamental, n_dividends: i64, from_date: Option<NaiveDate>) -> f64 {
        use chrono::{NaiveDate, Duration, DateTime};
        use chrono::Datelike; // Import the Datelike trait for date methods

        let today = from_date.unwrap_or_else(|| chrono::Local::now().date_naive());

        // Parse the dividend ex-date
        // The date format is expected to be like "2025-05-12T00:00:00Z"
        let div_ex_date = parse_date(fundamental.div_ex_date.as_str()).unwrap();

        // Calculate months to add based on dividend frequency
        // Quarterly: 12 / 4
        // Bi-annual: 12 / 2
        // Annual: 12
        let months_between_dividends = 12 / fundamental.div_freq;
        let captured_div = (fundamental.div_amount / fundamental.div_freq as f64) * n_dividends as f64;

        let mut months_until_next_call_event;
        let mut next_event_date;
        let mut next_div_ex_date;
        let mut final_div_ex_date;

        // If the ex-dividend date has been announced, and is after "today" then that's the first
        // possible dividend that can be captured
        if today < div_ex_date {
            let months_until_last_dividend_capture = max(months_between_dividends * (n_dividends - 1), 0);
            next_div_ex_date = div_ex_date;
            months_until_next_call_event = months_between_dividends * n_dividends;
            // months_until_next_call_event = months_between_dividends * n_dividends;
            next_event_date = div_ex_date.checked_add_months(Months::new(months_until_next_call_event as u32)).unwrap();
            final_div_ex_date = div_ex_date.checked_add_months(Months::new(months_until_last_dividend_capture as u32)).unwrap();
        } else {
            // even though we acquire the dividend at the ex date, it's not until the following "event" that the position could be called
            let months_until_next_dividend_capture = months_between_dividends * n_dividends;
            let months_until_last_dividend_capture = months_between_dividends * (n_dividends - 1);
            months_until_next_call_event = months_between_dividends * (n_dividends + 1);

            next_div_ex_date = div_ex_date.checked_add_months(Months::new(months_until_next_dividend_capture as u32)).unwrap();
            next_event_date = div_ex_date.checked_add_months(Months::new(months_until_next_call_event as u32)).unwrap();
            final_div_ex_date = div_ex_date.checked_add_months(Months::new(months_until_last_dividend_capture as u32)).unwrap();
            // You can't capture a dividend or get options called during the weekend
            next_div_ex_date = get_previous_weekday(next_div_ex_date);
        }

        final_div_ex_date = get_previous_weekday(final_div_ex_date);
        next_event_date = get_previous_weekday(next_event_date);

        // Parse the expiration date
        let expiration_date = parse_date(&self.expiration_date).unwrap();
        // Find the next event date
        let next_event_date = std::cmp::min(expiration_date, next_event_date);

        // The next event is either the next dividend date or the expiration date
        let days_to_next_event = (next_event_date - today).num_days();
        let days_to_next_ex_dividend = (next_div_ex_date - today).num_days();
        let days_to_final_ex_dividend = (final_div_ex_date - today).num_days();

        // Check conditions from
        if days_to_next_event <= 0 || (next_event_date - expiration_date).num_days() >= months_between_dividends * 30 {
            return 0.0;
        }

        let premium = match self.buy_write_premium(underlying_equity_price) {
            Some(premium) => premium,
            None => return 0.0,
        };
        let net = match self.buy_write_cost_basis(underlying_equity_price) {
            Some(cost_basis) => cost_basis,
            None => return 0.0, // Return 0.0 if we can't calculate the net
        };

        let return_after_dividend = ((((fundamental.div_pay_amount * (n_dividends as f64)) + premium) / net) / days_to_next_event as f64) * 365.0;
        debug!("{}", "-".repeat(70));
        debug!("Calculating return for {} after {} dividend", self.symbol, n_dividends);
        debug!("{}", "-".repeat(70));
        debug!("{:<25} {}", "Today:", today);
        debug!("{:<25} {}", "Expiration date:", expiration_date);
        debug!("{:<25} {}", "Initial dividend:", div_ex_date);
        debug!("{:<25} {} (In {} days)", "First dividend date:", next_div_ex_date, days_to_next_ex_dividend);
        debug!("{:<25} {} (In {} days)", "Last dividend date:", final_div_ex_date, days_to_final_ex_dividend);
        debug!("{:<25} {} (In {} days)", "Next event date:", next_event_date, days_to_next_event);
        debug!("{:<25} ${:.2}", "Option mid point:", self.mid().unwrap_or(0.0));
        debug!("{:<25} ${:.2}", "Captured dividend:", captured_div);
        debug!("{:<25} ${:.2}", "Equity price:", underlying_equity_price);
        debug!("{:<25} ${:.2}", "Equity yearly dividend:", fundamental.div_amount);
        debug!("{:<25} {:.2}%", "Buy write insurance:", self.buy_write_insurance(underlying_equity_price).unwrap_or(0.0) * 100.0);
        debug!("{:<25} ${:.2}", "Buy write premium:", premium);
        debug!("{:<25} ${:.2}", "Buy write cost basis:", net);
        debug!("{:<25} {:.2}%", format!("Return after {} dividend:", n_dividends), return_after_dividend*100.0);
        return_after_dividend
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use lazy_static::lazy_static;
    use crate::test_utils;
    use crate::test_utils::load_test_quote_data;

    lazy_static!(
        static ref SAMPLE_EQUITY_PRICE: f64 = 207.93;
    );
    fn get_test_sample_contract() -> OptionContract {
        serde_json::from_str(&r#"
        {
          "putCall": "CALL",
          "symbol": "AAPL  270617C00165000",
          "description": "AAPL 06/17/2027 165.00 C",
          "exchangeName": "OPR",
          "bidPrice": null,
          "askPrice": null,
          "lastPrice": null,
          "markPrice": null,
          "bidSize": 10,
          "askSize": 11,
          "lastSize": 1,
          "highPrice": 0.0,
          "lowPrice": 0.0,
          "openPrice": 0.0,
          "closePrice": 70.13,
          "totalVolume": 0,
          "tradeDate": null,
          "quoteTimeInLong": 1747425600089,
          "tradeTimeInLong": 1747322585191,
          "netChange": 0.74,
          "volatility": 31.42,
          "delta": 0.803,
          "gamma": 0.003,
          "theta": -0.023,
          "vega": 0.782,
          "rho": 2.138,
          "timeValue": 24.61,
          "openInterest": 0,
          "isInTheMoney": null,
          "theoreticalOptionValue": 67.077,
          "theoreticalVolatility": 29.0,
          "isMini": null,
          "isNonStandard": null,
          "optionDeliverablesList": [
            {
              "symbol": "AAPL",
              "assetType": "STOCK",
              "deliverableUnits": 100.0,
              "currencyType": null
            }
          ],
          "strikePrice": 165.0,
          "expirationDate": "2027-06-17T20:00:00.000+00:00",
          "daysToExpiration": 761.0,
          "expirationType": "S",
          "lastTradingDay": 1813276800000,
          "multiplier": 100.0,
          "settlementType": "P",
          "deliverableNote": "100 AAPL",
          "isIndexOption": null,
          "percentChange": 1.06,
          "markChange": -0.31
        }
            "#).unwrap()
    }


    #[test]
    fn test_chains_response_deserialization() {
        let chains = test_utils::load_test_chains_data();
        assert_eq!(chains.symbol, "AAPL");
        assert_eq!(chains.status, "SUCCESS");
        assert_eq!(chains.strategy, "COVERED");
        assert_eq!(chains.underlying.ask, 211.91);
        assert_eq!(chains.underlying.bid, 211.85);
    }

    #[derive(Debug)]
    struct ContractAnalysis {
        contract: String,
        expiration_date: String,
        strike_price: String,
        equity_price: f64,
        contract_price: f64,
        time_value: f64,
        theoretical_value: f64,
        net_position: f64,
        insurance: f64,
        premium: f64,
        return_1_div: f64,
        return_2_div: f64,
        return_3_div: f64,
        return_4_div: f64,
        return_5_div: f64,
    }

    #[test]
    fn test_return_calculations() {
        let chains = test_utils::load_test_chains_data();
        let quote = test_utils::load_test_quote_data();

        println!("{:<30} {:<8} {:<8} {:<8} {:<8} {:<8} {:<7} {:<8} {:<8} {:<8} {:<8} {:<8} {:<8}",
                 "Contract", "Equity", "Price", "TimeVal", "TheoVal", "Net", "Insur", "Prem", "Ret1", "Ret2", "Ret3", "Ret4", "Ret5");
        println!("{:-<140}", "");

        for (expiration_date, strikes) in chains.call_exp_date_map {
            // This comes in the format of `2025-07-18:56`
            let expiration_date: String = expiration_date.split(':').collect();
            for (strike, contracts) in strikes {
                for contract in contracts {
                    if contract.should_ignore(quote.quote.last_price).unwrap_or(true) {
                        continue;
                    }

                    let analysis = ContractAnalysis {
                        contract: contract.description.clone(),
                        expiration_date: expiration_date.clone(),
                        strike_price: strike.clone(),
                        equity_price: quote.quote.last_price,
                        contract_price: contract.mid().unwrap_or(0.0),
                        time_value: contract.time_value,
                        theoretical_value: contract.theoretical_option_value,
                        net_position: contract.buy_write_cost_basis(quote.quote.last_price).unwrap(),
                        insurance: contract.buy_write_insurance(quote.quote.last_price).unwrap() * 100.0,
                        premium: contract.buy_write_premium(quote.quote.last_price).unwrap(),
                        return_1_div: contract.calculate_return_after_dividend(quote.quote.last_price, quote.fundamental.clone(), 1, None),
                        return_2_div: contract.calculate_return_after_dividend(quote.quote.last_price, quote.fundamental.clone(), 2, None),
                        return_3_div: contract.calculate_return_after_dividend(quote.quote.last_price, quote.fundamental.clone(), 3, None),
                        return_4_div: contract.calculate_return_after_dividend(quote.quote.last_price, quote.fundamental.clone(), 4, None),
                        return_5_div: contract.calculate_return_after_dividend(quote.quote.last_price, quote.fundamental.clone(), 5, None),
                    };

                    println!("{:<30} ${:<7.2} ${:<7.2} ${:<7.2} ${:<7.2} ${:<7.2} {:<7} ${:<7.2} {:<7} {:<7} {:<7} {:<7} {:<7}",
                             analysis.contract,
                             analysis.equity_price,
                             analysis.contract_price,
                             analysis.time_value,
                             analysis.theoretical_value,
                             analysis.net_position,
                             format!("{:.2}%", analysis.insurance),
                             analysis.premium,
                             format!("{:.2}%", analysis.return_1_div * 100.0),
                             format!("{:.2}%", analysis.return_2_div * 100.0),
                             format!("{:.2}%", analysis.return_3_div * 100.0),
                             format!("{:.2}%", analysis.return_4_div * 100.0),
                             format!("{:.2}%", analysis.return_5_div * 100.0)
                    );
                }
            }
        }
    }

    #[test]
    /// Should match the close price when the bid / ask values are null
    fn test_mid_point_calculation() {
        let contract = get_test_sample_contract();
        assert_eq!(56.38, contract.mid().unwrap())
    }
    #[test]
    fn test_cost_basis_calculation() {
        let contract = get_test_sample_contract();
        assert_eq!(
            151.55,
            contract.buy_write_cost_basis(*SAMPLE_EQUITY_PRICE).unwrap()
        )
    }
    #[test]
    fn test_premium_calculation() {
        let contract = get_test_sample_contract();
        assert_eq!(
            3.45,
            round_to_decimals(contract.buy_write_premium(*SAMPLE_EQUITY_PRICE).unwrap(), Some(2.0))
        )
    }

    #[test]
    fn test_insurance_calculation() {
        let contract = get_test_sample_contract();
        assert_eq!(
            0.27,
            round_to_decimals(contract.buy_write_insurance(*SAMPLE_EQUITY_PRICE).unwrap(), Some(2.0))
        )
    }

    #[test]
    fn test_calculate_return_after_dividend() {
        let contract = get_test_sample_contract();
        let quote = load_test_quote_data();
        let fundamental = quote.fundamental;
        let from_date = NaiveDate::from_ymd_opt(2025, 5, 19);
        println!("Calculating return after dividend for {}", contract.symbol);
        println!("Underlying equity price: {}", *SAMPLE_EQUITY_PRICE);
        println!("Ex Div date: {:?}", fundamental.div_ex_date);
        assert_eq!(
            round_to_decimals(contract.calculate_return_after_dividend(*SAMPLE_EQUITY_PRICE, fundamental.clone(), 1, from_date), Some(4.0)),
            0.836
        );
        assert_eq!(
            round_to_decimals(contract.calculate_return_after_dividend(*SAMPLE_EQUITY_PRICE, fundamental.clone(), 2, from_date), Some(4.0)),
            0.4102
        );
        assert_eq!(
            round_to_decimals(contract.calculate_return_after_dividend(*SAMPLE_EQUITY_PRICE, fundamental.clone(), 3, from_date), Some(4.0)),
            0.2735
        );
        assert_eq!(
            round_to_decimals(contract.calculate_return_after_dividend(*SAMPLE_EQUITY_PRICE, fundamental.clone(), 4, from_date), Some(4.0)),
            0.2078
        );
        assert_eq!(
            round_to_decimals(contract.calculate_return_after_dividend(*SAMPLE_EQUITY_PRICE, fundamental.clone(), 5, from_date), Some(4.0)),
            0.167
        );
    }
}
