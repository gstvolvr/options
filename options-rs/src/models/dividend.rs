use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Dividend {
    pub dividend_amount: String,
    pub dividend_currency: String,
    pub dividend_declared_date: String,
    pub dividend_description: String,
    pub dividend_ex_date: i64,
    pub dividend_flag: String,
    pub dividend_frequency: String,
    pub dividend_payment_date: String,
    pub dividend_record_date: String,
    pub dividend_refid: String,
    pub dividend_symbol: String,
    pub dividend_id: String,
    pub dividend_key: String,
    pub dividend_subkey: String,
    pub dividend_date: String,
    pub dividend_updated: String,
    pub dividend_calculated: String,
    pub gross_annual_yield: String
}
