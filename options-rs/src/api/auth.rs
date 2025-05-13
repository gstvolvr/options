use url::Url;
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{url, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl};
use reqwest::{Client, Error};
use std::env;
use std::sync::{Arc, Mutex};
use tokio;
use tokio::io::{self, AsyncBufReadExt};
use crate::api::server::run_redirect_server;

pub const MARKET_DATA_API_URL: &str = "https://api.schwabapi.com/marketdata/v1";
pub const TOKEN_URL: &str = "https://api.schwabapi.com/v1/oauth/token";
// pub const REDIRECT_URL: &str = "https://developer.schwab.com/oauth2-redirect.html";
pub const REDIRECT_URL: &str = "http://127.0.0.1:8080";
pub const AUTH_URL: &str = "https://api.schwabapi.com/v1/oauth/authorize";

#[derive(Clone)]
pub struct OAuthClient {
    token: Arc<Mutex<String>>,
    client: Client,
}

impl OAuthClient {
    pub fn new(token: String) -> Self {
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

fn extract_code(url: String) -> Result<String, &'static str> {
    match Url::parse(url.trim_end_matches('\n')) {
        Ok(parsed_url) => {
            parsed_url
                .query_pairs()
                .find(|(key, _)| key == "code")
                .map(|(_, value)| value.to_string())
                .ok_or("Code parameter not found")
        }
        Err(_) => Err("Invalid URL"),
    }
}

async fn read_input() -> String {
    let mut response_url = String::new();
    let mut stdin = io::BufReader::new(io::stdin());
    stdin.read_line(&mut response_url).await.unwrap();
    response_url
}

// Follow OAuth flow
pub async fn get_initial_token() -> Result<String, Box<dyn std::error::Error>> {
    let client_id = ClientId::new(env::var("SCHWAB_CLIENT_ID").expect("Missing CLIENT_ID"));
    let client_secret = ClientSecret::new(env::var("SCHWAB_CLIENT_SECRET").expect("Missing CLIENT_SECRET"));
    let auth_url = AuthUrl::new(AUTH_URL.to_string())?;
    let token_url = TokenUrl::new(TOKEN_URL.to_string())?;
    let redirect_url = RedirectUrl::new(format!("{}/callback", REDIRECT_URL))?;
    println!("Auth URL: {:?}", redirect_url);
    println!("Redirect URL: {:?}", redirect_url);
    println!("Token URL: {:?}", token_url);

    // Create OAuth client
    let client = BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url)).set_redirect_uri(redirect_url);

    // Generate CSRF token and authorization URL
    let csrf_state = CsrfToken::new_random();
    let (auth_url, csrf_token) = client
        .authorize_url(|| csrf_state)  // Use the same CSRF token
        .add_scope(Scope::new("readonly".to_string()))
        .url();

    // Create a channel to receive the authorization code
    let (code_sender, code_receiver) = tokio::sync::oneshot::channel();

    // Start the redirect server
    let server_handle = tokio::spawn(run_redirect_server(code_sender));

    // Give the server a moment to start
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    println!("Server started at: {}", REDIRECT_URL);
    println!("Browse to: {}", auth_url);

    // Wait for the authorization code
    let auth_code = code_receiver.await?;

    // Exchange the authorization code for an access token
    let token_result = client
        .exchange_code(AuthorizationCode::new(auth_code))
        .request_async(async_http_client)
        .await?;

    // Close server connection
    server_handle.abort();

    println!("Initial token: {}", token_result.access_token().secret());
    Ok(token_result.access_token().secret().to_string())
}

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // Get the initial token
//     let initial_token = get_initial_token().await?;
//     let oauth_client = OAuthClient::new(initial_token);
//
//     // Make an authenticated request
//     // let response = oauth_client.get("https://api.example.com/data").await?;
//     // println!("Response: {:?}", response);
//     //
//     // // Refresh the token when needed
//     // let new_token = "new_access_token".to_string();
//     // oauth_client.refresh_token(new_token).await;
//     //
//     // // Make another authenticated request with the new token
//     // let response = oauth_client.get("https://api.example.com/data").await?;
//     // println!("Response: {:?}", response);
//     //
//     // Ok(())
// }
//
