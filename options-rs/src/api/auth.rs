//! Schwab API OAuth 2 Authentication
//! Schwab Documentation: https://developer.schwab.com/user-guides/get-started/authenticate-with-oauth

use url::Url;
use oauth2::reqwest::async_http_client;
use oauth2::{url, AuthUrl, AuthorizationCode, basic::BasicClient, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl, RefreshToken, RefreshTokenRequest};
use reqwest::{Client, Error};
use std::env;
use std::sync::{Arc, Mutex};
use tokio;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener};
use std::process::Command;
use lazy_static::lazy_static;
use std::fs::File;
use std::future::Future;
use std::path::Path;
use rustls::{Certificate as RustlsCertificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys, rsa_private_keys};
use tokio_rustls::TlsAcceptor;
use std::io::BufReader;
use std::pin::Pin;
use crate::api::quote;
use crate::config::{SSL_CERT_KEY_PATH, SSL_CERT_PATH, CLOUD_PROJECT_ID};
use google_cloud_gax::paginator::ItemPaginator as _;
use google_cloud_secretmanager_v1::client::SecretManagerService;
use google_cloud_secretmanager_v1::model::{Secret, SecretPayload};
use log::debug;
use crate::api;
use crate::api::schwab::quote;

pub const MARKET_DATA_API_URL: &str = "https://api.schwabapi.com/marketdata/v1";
pub const TOKEN_URL: &str = "https://api.schwabapi.com/v1/oauth/token";
pub const REDIRECT_URL: &str = "https://127.0.0.1:8080/callback";
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

    pub async fn get(&self, url: &str) -> Result<reqwest::Response, Error> {
        let token = self.token.lock().unwrap().clone();
        let request = self.client
            .get(url)
            .bearer_auth(token)
            .header("accept", "application/json")
            .send();

        request.await
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

/// Open the default browser with the given URL
fn open_browser(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(["/c", "start", url])
            .spawn()?;
    }
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(url)
            .spawn()?;
    }
    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(url)
            .spawn()?;
    }
    Ok(())
}

// Create a static variable to store the authorization code
lazy_static! {
    static ref AUTH_CODE: Mutex<Option<String>> = Mutex::new(None);
}

/// Get SSL certificates created by mkcert
fn get_certificate_paths() -> Result<(String, String), Box<dyn std::error::Error>> {
    // Check if certificate files exist
    if !Path::new(SSL_CERT_PATH).exists() || !Path::new(SSL_CERT_KEY_PATH).exists() {
        return Err(format!("Certificate files not found: {} and {}. Please create them using mkcert.", SSL_CERT_PATH, SSL_CERT_KEY_PATH).into());
    }

    Ok((SSL_CERT_PATH.to_string(), SSL_CERT_KEY_PATH.to_string()))
}

// Load TLS configuration from certificate and key files
async fn load_tls_config(cert_path: &str, key_path: &str) -> Result<ServerConfig, Box<dyn std::error::Error>> {
    // Load certificate
    let cert_file = File::open(cert_path)?;
    let mut cert_reader = BufReader::new(cert_file);
    let cert_chain = certs(&mut cert_reader)?
        .into_iter()
        .map(RustlsCertificate)
        .collect();

    // Load private key
    let key_file = File::open(key_path)?;
    let mut key_reader = BufReader::new(key_file);

    // Try to load as PKCS#8 first
    let mut keys = pkcs8_private_keys(&mut key_reader)?;

    // If no PKCS#8 keys found, try RSA format
    if keys.is_empty() {
        // Reset the reader position
        let key_file = File::open(key_path)?;
        let mut key_reader = BufReader::new(key_file);

        // Try to load as RSA
        keys = rsa_private_keys(&mut key_reader)?;

        if keys.is_empty() {
            return Err("No private keys found in either PKCS#8 or RSA format".into());
        }
    }

    let key = PrivateKey(keys.remove(0));

    // Create server config
    let mut config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(cert_chain, key)?;

    Ok(config)
}

// Start a local server to listen for the OAuth callback
async fn start_callback_server() -> Result<String, Box<dyn std::error::Error>> {
    // Reset the authorization code
    {
        let mut code = AUTH_CODE.lock().unwrap();
        *code = None;
    }

    // Get certificate paths
    let (cert_path, key_path) = get_certificate_paths()?;

    // Load TLS configuration
    let tls_config = load_tls_config(&cert_path, &key_path).await?;
    let tls_acceptor = TlsAcceptor::from(Arc::new(tls_config));

    // Create a TCP listener
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Listening for callback on https://127.0.0.1:8080/callback");

    // Wait for a connection
    loop {
        let (socket, _) = listener.accept().await?;

        // Accept TLS connection
        let tls_acceptor = tls_acceptor.clone();
        let tls_stream = match tls_acceptor.accept(socket).await {
            Ok(stream) => stream,
            Err(e) => {
                eprintln!("Failed to accept TLS connection: {}", e);
                continue;
            }
        };

        let (mut reader, mut writer) = tokio::io::split(tls_stream);

        // Read the request
        let mut buffer = [0; 1024];
        let n = reader.read(&mut buffer).await?;
        let request = String::from_utf8_lossy(&buffer[..n]);

        // Check if it's a GET request to /callback
        if request.starts_with("GET /callback") {
            // Extract the code from the query parameters
            if let Some(query) = request.split_whitespace().nth(1) {
                if let Ok(url) = Url::parse(&format!("https://localhost{}", query)) {
                    if let Some((_, code)) = url.query_pairs().find(|(key, _)| key == "code") {
                        // Store the code
                        {
                            let mut auth_code = AUTH_CODE.lock().unwrap();
                            *auth_code = Some(code.to_string());
                        }

                        // Send a success response
                        let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n\
                            <html><body><h1>Authentication Successful!</h1>\
                            <p>You can now close this window and return to the application.</p>\
                            </body></html>";
                        writer.write_all(response.as_bytes()).await?;

                        // Break the loop
                        break;
                    }
                }
            }

            // Send an error response if code extraction failed
            let response = "HTTP/1.1 400 Bad Request\r\nContent-Type: text/html\r\n\r\n\
                <html><body><h1>Authentication Failed</h1>\
                <p>No authorization code received.</p>\
                </body></html>";
            writer.write_all(response.as_bytes()).await?;
        }
    }

    // Return the authorization code
    let code = AUTH_CODE.lock().unwrap();
    match code.clone() {
        Some(code) => Ok(code),
        None => Err("No authorization code received".into()),
    }
}

pub async fn authenticate() -> Result<OAuthClient, Box<dyn std::error::Error>> {
    authenticate_with_retry(true, false, 0).await
}

#[allow(clippy::boxed_local)]
async fn authenticate_with_retry(with_refresh: bool, with_schwab: bool, try_number: i32) -> Result<OAuthClient, Box<dyn std::error::Error>> {
    let client = SecretManagerService::builder().build().await?;
    debug!("Authentication attempt number: {try_number}");

    let mut items = client
        .list_secrets()
        .set_parent(format!("projects/{CLOUD_PROJECT_ID}"))
        .by_item();

    let mut access_secret = None;
    let mut refresh_secret = None;
    while let Some(item) = items.next().await {
        let item = item?;
        debug!("Secret: {}", item.name);
        if item.name.contains("ACCESS") {
            access_secret = Some(item);
        } else if item.name.contains("REFRESH") {
            refresh_secret = Some(item);
        }
    }
    let access_secret = access_secret.ok_or("No access secret found")?;
    let refresh_secret = refresh_secret.ok_or("No refresh secret found")?;

    let access_version = client.access_secret_version();
    let access_version = access_version.set_name(&(access_secret.name.clone() + "/versions/latest").clone()).send().await?;
    let access_token_bytes = access_version.payload.ok_or("No payload")?.data;
    let mut access_token = String::from_utf8(access_token_bytes.to_vec())?;
    let refresh_version = client.access_secret_version().set_name(&(refresh_secret.name.clone() + "/versions/latest")).send().await?;
    let refresh_token_bytes = refresh_version.payload.ok_or("No payload")?.data;
    let refresh_token = String::from_utf8(refresh_token_bytes.to_vec())?;

    if with_schwab {
        let (access_token_str, refresh_token_str) = api::auth::get_tokens_from_schwab().await.expect("Failed to get token");
        let mut access_payload = SecretPayload::default();
        access_payload.data = access_token_str.as_bytes().to_vec().into();
        client.add_secret_version()
            .set_parent(access_secret.name)
            .set_payload(access_payload).send().await?;
        let mut refresh_payload = SecretPayload::default();
        refresh_payload.data = refresh_token_str.as_bytes().to_vec().into();
        client.add_secret_version()
            .set_parent(refresh_secret.name)
            .set_payload(refresh_payload).send().await?;
        debug!("New token obtained and saved");
    } else if with_refresh {
        access_token = get_refreshed_access_token_from_schwab(refresh_token).await?;
        let mut access_payload = SecretPayload::default();
        access_payload.data = access_token.as_bytes().to_vec().into();
        client.add_secret_version()
            .set_parent(access_secret.name)
            .set_payload(access_payload);
        debug!("Exchanged refresh token for new access token");
    }

    let oauth_client = OAuthClient::new(access_token);

    // Contact the Schwab API and see if our current access token works
    match quote("AAPL", &oauth_client).await {
        Ok(_) => {
            eprintln!("Successfully connected to API");
            Ok(oauth_client)
        },
        // If the access token fails then set the refresh token
        Err(e) => {
            eprintln!("Failed to connect to API: {}", e);
            // if authenticating with Schwab didn't work then raise an error
            if with_schwab {
                return Err(e.into());
            // if we haven't attempted to use the refresh token, try that first
            } else if !with_refresh {
                Box::pin(authenticate_with_retry(true, false, try_number + 1)).await
            // if refreshing the token fails, then fallback to doing authentication with Schwab
            } else if !with_schwab {
                Box::pin(authenticate_with_retry(false, true, try_number + 1)).await
            } else {
                Err(e.into())
            }
        },
    }
}

pub async fn get_refreshed_access_token_from_schwab(refresh_token: String) -> Result<String, Box<dyn std::error::Error>> {
    let client_id = ClientId::new(env::var("SCHWAB_CLIENT_ID").expect("Missing CLIENT_ID"));
    let client_secret = ClientSecret::new(env::var("SCHWAB_CLIENT_SECRET").expect("Missing CLIENT_SECRET"));
    let auth_url = AuthUrl::new(AUTH_URL.to_string())?;
    let token_url = TokenUrl::new(TOKEN_URL.to_string())?;

    let token_response = BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
        .exchange_refresh_token(&RefreshToken::new(refresh_token))
        .request_async(oauth2::reqwest::async_http_client)
        .await?;
    Ok(token_response.access_token().secret().to_string())
}

// Follow OAuth flow with automatic browser opening and callback handling
// This function will:
// 1. Generate an authorization URL
// 2. Open the user's default browser to that URL
// 3. Start a local server to listen for the OAuth callback
// 4. Exchange the authorization code for an access token
pub async fn get_tokens_from_schwab() -> Result<(String, String), Box<dyn std::error::Error>> {
    let client_id = ClientId::new(env::var("SCHWAB_CLIENT_ID").expect("Missing CLIENT_ID"));
    let client_secret = ClientSecret::new(env::var("SCHWAB_CLIENT_SECRET").expect("Missing CLIENT_SECRET"));
    let auth_url = AuthUrl::new(AUTH_URL.to_string())?;
    let token_url = TokenUrl::new(TOKEN_URL.to_string())?;
    let redirect_url = RedirectUrl::new(REDIRECT_URL.to_string())?;
    println!("Auth URL: {:?}", redirect_url);
    println!("Redirect URL: {:?}", redirect_url);
    println!("Token URL: {:?}", token_url);

    let client = BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
        .set_redirect_uri(redirect_url);

    // Generate the authorization URL
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("readonly".to_string()))
        .url();


    println!("Opening browser to: {}", auth_url);

    // Open the browser to the authorization URL
    open_browser(auth_url.as_ref())?;

    // Start the callback server in the background
    println!("Waiting for authorization...");

    // Wait for the callback with the authorization code
    let auth_code = start_callback_server().await?;
    println!("Auth code received: {}", auth_code);

    // Exchange the authorization code for an access token
    let token_result = client
        .exchange_code(AuthorizationCode::new(auth_code.to_string()))
        .request_async(async_http_client)
        .await?;

    // eprintln!("Initial token: {}", token_result.access_token().secret());
    let access_token = token_result.access_token().secret().to_string();
    let refresh_token = token_result.refresh_token().expect("refresh token").secret().to_string();
    Ok((access_token, refresh_token))
}
