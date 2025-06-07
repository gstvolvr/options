use lazy_static::lazy_static;

pub static DATA_DIR_PATH: &str = "../data";
pub static QUOTES_FILENAME: &str = "schwab_quotes.jsonl";
pub static CHAINS_FILENAME: &str = "schwab_chains.jsonl";
pub static RETURNS_FILENAME: &str = "schwab_returns.csv";
pub static RETURNS_JSON_FILENAME: &str = "schwab_returns.jsonl";
pub static SYMBOLS_FILENAME: &str = "symbols.csv"; // TODO: swap to `symbols.csv` when ready
pub static COMPANIES_FILENAME: &str = "companies.csv";
pub static SSL_CERT_PATH: &str = "../127.0.0.1.pem";
pub static SSL_CERT_KEY_PATH: &str = "../127.0.0.1-key.pem";
pub static CLOUD_PROJECT_ID: &str = "options-282500";

lazy_static! {
    pub static ref QUOTES_DATA_PATH: String = format!("{}/{}", DATA_DIR_PATH, QUOTES_FILENAME);
    pub static ref CHAINS_DATA_PATH: String = format!("{}/{}", DATA_DIR_PATH, CHAINS_FILENAME);
    pub static ref SYMBOLS_DATA_PATH: String = format!("{}/{}", DATA_DIR_PATH, SYMBOLS_FILENAME);
    pub static ref RETURNS_DATA_PATH: String = format!("{}/{}", DATA_DIR_PATH, RETURNS_FILENAME);
    pub static ref RETURNS_JSON_DATA_PATH: String = format!("{}/{}", DATA_DIR_PATH, RETURNS_JSON_FILENAME);
    pub static ref COMPANIES_DATA_PATH: String = format!("{}/{}", DATA_DIR_PATH, COMPANIES_FILENAME);
}
