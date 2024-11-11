use std::error::Error;
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use serde::de::DeserializeOwned;
use crate::api::quote::QuoteApiResponse;
use crate::api::chains::ChainsApiResponse;
use crate::api::auth::MARKET_DATA_API_URL;


pub async fn call_api<T: DeserializeOwned>(api_url: &str) -> Result<T, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();

    headers.insert(AUTHORIZATION, format!("Bearer {}", "your_token_here").parse()?);
    headers.insert(CONTENT_TYPE, "application/json".parse()?);

    let response = client
        .get(api_url)
        .headers(headers)
        .send()
        .await?;

    if response.status().is_success() {
        let body = response.text().await?;
        let api_response: T = serde_json::from_str(&body)?;
        Ok(api_response)
    } else {
        Err(format!("Request failed with status: {}", response.status()).into())
    }
}


pub async fn quote(symbol: &str) -> Result<QuoteApiResponse, Box<dyn Error>> {
    let api_url = format!("{}/quotes?symbol={}", MARKET_DATA_API_URL, symbol);
    let response = call_api::<QuoteApiResponse>(&api_url).await?;
    return Ok(response);
}

pub async fn chains(symbol: &str) -> Result<ChainsApiResponse, Box<dyn Error>> {
    let api_url = format!("{}/chains?symbols={}&fields=quote&indicative=false", MARKET_DATA_API_URL, symbol);
    let response: ChainsApiResponse = call_api(&api_url).await?;
    return Ok(response);
}
