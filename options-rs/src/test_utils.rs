use std::fs;
use std::path::Path;
use crate::api::chains::ChainsApiResponse;
use crate::api::quote::QuoteApiResponse;

pub(crate) const TEST_CHAINS_DATA_PATH: &str = "src/api/test_data/chains_test_data.json";
pub(crate) const TEST_QUOTE_DATA_PATH: &str = "src/api/test_data/quote_test_data.json";

/// Writes API data to local JSON files
pub fn write_test_data(quotes: QuoteApiResponse, chains: ChainsApiResponse) {
    std::fs::write(
        TEST_QUOTE_DATA_PATH,
        serde_json::to_string_pretty(&quotes).expect("Failed to serialize quotes"),
    ).expect("Failed to write file");

    std::fs::write(
        TEST_CHAINS_DATA_PATH,
        serde_json::to_string_pretty(&chains).expect("Failed to serialize chains"),
    ).expect("Failed to write file");
}


fn load_test_data<T: serde::de::DeserializeOwned>(file_path: &str) -> T {
    let test_data_path = Path::new(file_path);
    let file = fs::File::open(test_data_path)
        .expect("Failed to open test data file");
    serde_json::from_reader(file)
        .expect("Failed to parse JSON into expected type")
}

pub fn load_test_chains_data() -> ChainsApiResponse {
    load_test_data(TEST_CHAINS_DATA_PATH)
}

pub fn load_test_quote_data() -> QuoteApiResponse {
    load_test_data(TEST_QUOTE_DATA_PATH)
}
