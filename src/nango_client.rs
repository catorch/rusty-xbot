use reqwest::Client;
use std::collections::HashSet;
use std::env;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct NangoConfig {
    pub client_id: String,
    pub secret_key: String,
    pub callback_url: String,
}

const X_CLIENT_ID: &str = "RustXBot";
const NANGO_X_PROVIDER_CONFIG_KEY: &str = "twitter-v2";

lazy_static::lazy_static! {
    static ref DEFAULT_REQUIRED_X_OAUTH_SCOPES: HashSet<&'static str> = {
        let mut scopes = HashSet::new();
        scopes.insert("tweet.read");
        scopes.insert("users.read");
        scopes.insert("offline.access");
        scopes.insert("tweet.write");
        scopes
    };
}

#[derive(Debug)]
pub struct OAuth2Token {
    pub token_type: String,
    pub access_token: String,
    pub scope: String,
    pub expires_in: Option<i64>,
}

impl OAuth2Token {
    pub fn from_raw(raw: &serde_json::Value) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            token_type: raw["token_type"].as_str().unwrap_or("").to_string(),
            access_token: raw["access_token"].as_str().unwrap_or("").to_string(),
            scope: raw["scope"].as_str().unwrap_or("").to_string(),
            expires_in: raw["expires_in"].as_i64(),
        })
    }

    pub fn scopes(&self) -> HashSet<&str> {
        self.scope.split_whitespace().collect()
    }
}

// Load Nango configuration from environment variables
pub fn load_nango_config() -> Result<NangoConfig, Box<dyn Error>> {
    Ok(NangoConfig {
        client_id: env::var("NANGO_CLIENT_ID")?.trim().to_string(),
        secret_key: env::var("NANGO_SECRET_KEY")?.trim().to_string(),
        callback_url: env::var("NANGO_CALLBACK_URL")?.trim().to_string(),
    })
}

// Fetch an OAuth token from Nango
pub async fn fetch_nango_oauth_token(
    client: &Client,
    config: &NangoConfig,
) -> Result<OAuth2Token, Box<dyn Error>> {
    let url = format!(
        "https://api.nango.dev/connection/{}?provider_config_key={}",
        config.client_id, NANGO_X_PROVIDER_CONFIG_KEY
    );

    println!("{:?}", url);

    let res = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", config.secret_key))
        .send()
        .await?;

    // print!("{:?}", res);

    if res.status().is_success() {
        let json: serde_json::Value = res.json().await?;

        let token = OAuth2Token::from_raw(&json["credentials"]["raw"])?;
        Ok(token)
    } else {
        Err(format!("Failed to fetch Nango OAuth token: {}", res.status()).into())
    }
}
