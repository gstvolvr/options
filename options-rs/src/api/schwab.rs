use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE, Client};
use crate::api::quote::QuoteApiResponse;
use crate::api::chains::ChainsApiResponse;


pub async fn call_api(api_url: &str) -> Result<ApiResponse, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();

    headers.insert(AUTHORIZATION, format!("Bearer {}", "your_token_here").parse()?);
    headers.insert(CONTENT_TYPE, "application/json".parse()?);

    let reponse = client
        .get(api_url)
        .headers(headers)
        .send()
        .await?;

    if response.status().is_success() {
        let body = response.text().await?;
        let api_response: ApiResponse = serde_json::from_str(&body)?;
        Ok(api_response)
    } else {
        Err(format!("Request failed with status: {}", response.status()).into())
    }
}


pub async fn quote(symbol: &str) -> Result<ApiResponse, Box<dyn Error>> {
    let api_url = format!("{}/chains?symbol={}", MARKET_DATA_API_URL, symbol);
    let response: QuoteApiResponse = call_api(&api_url).await;
}

pub async fn chains(symbol: &str) -> Result<ApiResponse, Box<dyn Error>> {
    let api_url = format!("{}/quotes?symbols={}&fields=quote&indicative=false", MARKET_DATA_API_URL, symbol);
    let response: ChainsApiResponse = call_api(&api_url).await;
}
