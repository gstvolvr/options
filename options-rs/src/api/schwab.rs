use std::error::Error;
use crate::api::quote::QuoteApiResponse;
use crate::api::auth::{OAuthClient, MARKET_DATA_API_URL};
use crate::api::chains::ChainsApiResponse;

pub async fn quote(symbol: &str, oauth_client: &OAuthClient) -> Result<QuoteApiResponse, Box<dyn Error>> {
    let api_url = format!("{}/quotes?symbols={}&fields=quote,reference&indicative=false", MARKET_DATA_API_URL, symbol);
    println!("Retrieving data from: {}", &api_url);
    let response = oauth_client.get(&api_url).await?;

    if response.status().is_success() {
        let text = response.text().await?;
        let json: serde_json::Map<String, serde_json::Value> = serde_json::from_str(&text)?;

        // Take first entry since we only request one symbol
        if let Some((_symbol, quote_data)) = json.into_iter().next() {
            // println!("symbol: {:?}", _symbol);
            // println!("quote: {:?}", quote_data);
            let api_response: QuoteApiResponse = serde_json::from_value(quote_data)?;
            Ok(api_response)
        } else {
            Err("No quote data found in response".into())
        }
    } else {
        let status = response.status();
        let error_text = response.text().await?;
        println!("Error response body: {}", error_text);
        Err(format!("Request failed with status: {}", status).into())
    }
}

pub async fn chains(symbol: &str, oauth_client: &OAuthClient) -> Result<ChainsApiResponse, Box<dyn Error>> {
    let api_url = format!("{}/chains?symbol={}&contractType=CALL&includeUnderlyingQuote=true&strategy=COVERED&range=ITM&toDate=2026-05-14", MARKET_DATA_API_URL, symbol);
    println!("Retrieving data from: {}", &api_url);
    let response = oauth_client.get(&api_url).await?;

    if response.status().is_success() {
        let text = response.text().await?;
        println!("chains: {}", text);
        let api_response: ChainsApiResponse = serde_json::from_str(&text)?;
        println!("chains: {:?}", api_response);
        Ok(api_response)
    } else {
        let status = response.status();
        let error_text = response.text().await?;
        println!("Error response body: {}", error_text);
        Err(format!("Request failed with status: {}", status).into())
    }
}
