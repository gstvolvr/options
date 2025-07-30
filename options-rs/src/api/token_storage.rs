// Complements of AI
use chrono::{Utc, Duration, DateTime};
use std::fs;
use std::path::PathBuf;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce
};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

#[derive(Serialize, Deserialize)]
pub struct TokenData {
    pub token: String,
    pub expiry: DateTime<Utc>,
}

pub struct TokenStorage {
    file_path: PathBuf,
    key: Key<Aes256Gcm>,
}

impl TokenStorage {
    pub fn new() -> Self {
        // Create config directory if it doesn't exist
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("options-rs");

        fs::create_dir_all(&config_dir).expect("Failed to create config directory");

        let file_path = config_dir.join("token.dat");

        // Create or load encryption key
        let key_path = config_dir.join("key.dat");
        let key = if key_path.exists() {
            // Load existing key
            let key_data = fs::read(&key_path).expect("Failed to read key file");
            Key::<Aes256Gcm>::from_slice(&key_data).clone()
        } else {
            // Generate new key
            let key = Aes256Gcm::generate_key(OsRng);
            fs::write(&key_path, key.as_slice()).expect("Failed to write key file");
            key
        };

        TokenStorage { file_path, key }
    }

    pub fn get_token(&self) -> Option<String> {
        if !self.file_path.exists() {
            return None;
        }

        // Read encrypted data
        let encrypted_data = match fs::read(&self.file_path) {
            Ok(data) => data,
            Err(_) => return None,
        };

        // Decode base64
        let decoded_data = match BASE64.decode(&encrypted_data) {
            Ok(data) => data,
            Err(_) => return None,
        };

        // Split nonce and ciphertext
        if decoded_data.len() <= 12 {
            return None;
        }

        let nonce_bytes = &decoded_data[0..12];
        let ciphertext = &decoded_data[12..];

        let nonce = Nonce::from_slice(nonce_bytes);

        // Decrypt
        let cipher = Aes256Gcm::new(&self.key);
        let plaintext = match cipher.decrypt(nonce, ciphertext) {
            Ok(data) => data,
            Err(_) => return None,
        };

        // Deserialize
        let token_data: TokenData = match serde_json::from_slice(&plaintext) {
            Ok(data) => data,
            Err(_) => return None,
        };

        // Check if token is expired
        if token_data.expiry < Utc::now() {
            return None;
        }

        Some(token_data.token)
    }

    pub fn save_token(&self, token: String) {
        // Create token data with expiry (20 minutes from now)
        let token_data = TokenData {
            token,
            expiry: Utc::now() + Duration::minutes(20),
        };

        // Serialize
        let plaintext = serde_json::to_vec(&token_data).expect("Failed to serialize token data");

        // Encrypt
        let cipher = Aes256Gcm::new(&self.key);
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let ciphertext = cipher.encrypt(&nonce, plaintext.as_ref())
            .expect("Failed to encrypt token data");

        // Combine nonce and ciphertext
        let mut combined = Vec::with_capacity(nonce.len() + ciphertext.len());
        combined.extend_from_slice(&nonce);
        combined.extend_from_slice(&ciphertext);

        // Encode as base64
        let encoded = BASE64.encode(combined);

        // Write to file
        fs::write(&self.file_path, encoded).expect("Failed to write token file");
    }
}

// Create a static instance of TokenStorage
pub static TOKEN_STORAGE: Lazy<TokenStorage> = Lazy::new(|| {
    TokenStorage::new()
});