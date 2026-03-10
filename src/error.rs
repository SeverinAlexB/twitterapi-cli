use thiserror::Error;

#[derive(Error, Debug)]
pub enum TwitterApiError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("API error: {0}")]
    Api(String),

    #[error("Rate limited (429). Retries exhausted.")]
    RateLimited,

    #[error("Authentication error: {0}")]
    AuthError(String),

    #[error("Cache error: {0}")]
    Cache(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}
