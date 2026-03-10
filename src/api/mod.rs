pub mod trends;
pub mod tweet;
pub mod user;

use crate::error::TwitterApiError;
use reqwest::Client;
use serde::de::DeserializeOwned;
use std::time::{Duration, Instant};
use tokio::time::sleep;

const BASE_URL: &str = "https://api.twitterapi.io";

pub struct TwitterApiClient {
    client: Client,
    api_key: String,
    last_request: std::sync::Mutex<Option<Instant>>,
}

impl TwitterApiClient {
    pub fn new(api_key: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to build HTTP client");

        Self {
            client,
            api_key,
            last_request: std::sync::Mutex::new(None),
        }
    }

    /// Rate limit: 200ms between requests
    async fn rate_limit(&self) {
        let min_interval = Duration::from_millis(200);
        let wait = {
            let mut last = self.last_request.lock().unwrap();
            let now = Instant::now();
            match *last {
                Some(t) => {
                    let elapsed = now.duration_since(t);
                    if elapsed < min_interval {
                        *last = Some(t + min_interval);
                        Some(min_interval - elapsed)
                    } else {
                        *last = Some(now);
                        None
                    }
                }
                None => {
                    *last = Some(now);
                    None
                }
            }
        };
        if let Some(wait) = wait {
            tracing::debug!("Rate limiting: waiting {}ms", wait.as_millis());
            sleep(wait).await;
        }
    }

    pub async fn get_with_retry(
        &self,
        path: &str,
        params: &[(&str, String)],
    ) -> Result<String, TwitterApiError> {
        let url = format!("{}{}", BASE_URL, path);
        let mut rate_limit_retries = 0u32;
        let mut server_error_retries = 0u32;

        loop {
            self.rate_limit().await;

            let response = self
                .client
                .get(&url)
                .header("X-API-Key", &self.api_key)
                .query(params)
                .send()
                .await?;
            let status = response.status();

            if status.is_success() {
                return Ok(response.text().await?);
            }

            if status.as_u16() == 401 || status.as_u16() == 403 {
                let body = response.text().await.unwrap_or_default();
                return Err(TwitterApiError::AuthError(format!(
                    "Authentication failed ({}): {}",
                    status, body
                )));
            }

            if status.as_u16() == 429 && rate_limit_retries < 3 {
                rate_limit_retries += 1;
                tracing::warn!("Rate limited, retrying in 1s ({}/3)", rate_limit_retries);
                sleep(Duration::from_secs(1)).await;
                continue;
            }

            if status.is_server_error() && server_error_retries < 1 {
                server_error_retries += 1;
                tracing::warn!("Server error {}, retrying in 2s", status);
                sleep(Duration::from_secs(2)).await;
                continue;
            }

            if status.as_u16() == 429 {
                return Err(TwitterApiError::RateLimited);
            }

            let body = response.text().await.unwrap_or_default();
            return Err(TwitterApiError::Api(format!(
                "API returned status {}: {}",
                status, body
            )));
        }
    }

    pub async fn get_json<T: DeserializeOwned>(
        &self,
        path: &str,
        params: &[(&str, String)],
    ) -> Result<crate::model::ApiResponse<T>, TwitterApiError> {
        let text = self.get_with_retry(path, params).await?;
        tracing::debug!("Response: {}", &text[..text.len().min(500)]);
        let resp: crate::model::ApiResponse<T> = serde_json::from_str(&text)?;
        if resp.status != "success" {
            let msg = resp.msg.unwrap_or_else(|| "Unknown error".to_string());
            return Err(TwitterApiError::Api(msg));
        }
        Ok(resp)
    }
}
