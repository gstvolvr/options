use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse,
    TokenUrl,
};
use reqwest::{Client, Error};
use std::env;
use std::sync::{Arc, Mutex};
use tokio;

const MARKET_DATA_API_URL: &str = "https://api.schwabapi.com/marketdata/v1";
const TOKEN_URL: &str = "https://api.schwabapi.com/v1/oauth/token";
const REDIRECT_URL: &str = "https://developer.schwab.com/oauth2-redirect.html";
const AUTH_URL: &str = "https://api.schwabapi.com/v1/oauth/authorize";

#[derive(Clone)]
struct OAuthClient {
    token: Arc<Mutex<String>>,
    client: Client,
}

impl OAuthClient {
    fn new(token: String) -> Self {
        OAuthClient {
            token: Arc::new(Mutex::new(token)),
            client: Client::new(),
        }
    }

    async fn refresh_token(&self, new_token: String) {
        let mut token = self.token.lock().unwrap();
        *token = new_token;
    }

    async fn get(&self, url: &str) -> Result<reqwest::Response, Error> {
        let token = self.token.lock().unwrap().clone();
        self.client
            .get(url)
            .bearer_auth(token)
            .send()
            .await
    }
}

async fn get_initial_token() -> Result<String, Box<dyn std::error::Error>> {
    let client_id = ClientId::new(env::var("SCHWAB_CLIENT_ID").expect("Missing CLIENT_ID"));
    let client_secret = ClientSecret::new(env::var("SCHWAB_CLIENT_SECRET").expect("Missing CLIENT_SECRET"));
    let auth_url = AuthUrl::new(AUTH_URL.to_string())?;
    let token_url = TokenUrl::new(TOKEN_URL.to_string())?;
    let redirect_url = RedirectUrl::new(REDIRECT_URL.to_string())?;

    let client = BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
        .set_redirect_uri(redirect_url);

    // Generate the authorization URL
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("repo".to_string()))
        .url();

    println!("Browse to: {}", auth_url);

    // Simulate the user browsing to the URL and providing the authorization code
    // In a real application, you'd redirect the user to the URL and then handle the callback
    println!("Enter the authorization code:");
    let mut auth_code = String::new();
    std::io::stdin().read_line(&mut auth_code)?;
    let auth_code = auth_code.trim();

    // Exchange the authorization code for an access token
    let token_result = client
        .exchange_code(AuthorizationCode::new(auth_code.to_string()))
        .request_async(async_http_client)
        .await?;

    Ok(token_result.access_token().secret().to_string())
}

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // Get the initial token
//     let initial_token = get_initial_token().await?;
//     let oauth_client = OAuthClient::new(initial_token);
//
//     // Make an authenticated request
//     let response = oauth_client.get("https://api.example.com/data").await?;
//     println!("Response: {:?}", response);
//
//     // Refresh the token when needed
//     let new_token = "new_access_token".to_string();
//     oauth_client.refresh_token(new_token).await;
//
//     // Make another authenticated request with the new token
//     let response = oauth_client.get("https://api.example.com/data").await?;
//     println!("Response: {:?}", response);
//
//     Ok(())
// }
//