use crate::error::TwitterApiError;
use serde::{de::DeserializeOwned, Serialize};
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

pub struct Cache {
    dir: PathBuf,
    read_enabled: bool,
}

pub struct CacheHit<T> {
    pub data: T,
    pub cached_at: SystemTime,
}

/// Per-resource TTLs
const TTL_USER_PROFILE: Duration = Duration::from_secs(24 * 60 * 60); // 1 day
const TTL_TWEET: Duration = Duration::from_secs(7 * 24 * 60 * 60); // 7 days
const TTL_SEARCH: Duration = Duration::from_secs(60 * 60); // 1 hour
const TTL_TRENDS: Duration = Duration::from_secs(5 * 60); // 5 minutes
const TTL_FOLLOWERS: Duration = Duration::from_secs(60 * 60); // 1 hour

impl Cache {
    pub fn new(cache_dir: PathBuf, no_cache: bool) -> Self {
        Self {
            dir: cache_dir,
            read_enabled: !no_cache,
        }
    }

    // --- User profile ---
    pub fn get_user(&self, username: &str) -> Option<CacheHit<serde_json::Value>> {
        if !self.read_enabled {
            return None;
        }
        let path = self
            .dir
            .join(format!("user_{}.json", username.to_lowercase()));
        self.read_cached(&path, TTL_USER_PROFILE)
    }

    pub fn set_user(&self, username: &str, data: &impl Serialize) -> Result<(), TwitterApiError> {
        let path = self
            .dir
            .join(format!("user_{}.json", username.to_lowercase()));
        self.write_cached(&path, data)
    }

    // --- Tweet ---
    pub fn get_tweet<T: DeserializeOwned>(&self, tweet_id: &str) -> Option<CacheHit<T>> {
        if !self.read_enabled {
            return None;
        }
        let path = self.dir.join(format!("tweet_{}.json", tweet_id));
        self.read_cached(&path, TTL_TWEET)
    }

    pub fn set_tweet(&self, tweet_id: &str, data: &impl Serialize) -> Result<(), TwitterApiError> {
        let path = self.dir.join(format!("tweet_{}.json", tweet_id));
        self.write_cached(&path, data)
    }

    // --- Search ---
    pub fn get_search<T: DeserializeOwned>(&self, cache_key: &str) -> Option<CacheHit<T>> {
        if !self.read_enabled {
            return None;
        }
        let path = self.dir.join(format!("search_{}.json", cache_key));
        self.read_cached(&path, TTL_SEARCH)
    }

    pub fn set_search(
        &self,
        cache_key: &str,
        data: &impl Serialize,
    ) -> Result<(), TwitterApiError> {
        let path = self.dir.join(format!("search_{}.json", cache_key));
        self.write_cached(&path, data)
    }

    // --- Trends ---
    pub fn get_trends<T: DeserializeOwned>(&self, woeid: u64) -> Option<CacheHit<T>> {
        if !self.read_enabled {
            return None;
        }
        let path = self.dir.join(format!("trends_{}.json", woeid));
        self.read_cached(&path, TTL_TRENDS)
    }

    pub fn set_trends(&self, woeid: u64, data: &impl Serialize) -> Result<(), TwitterApiError> {
        let path = self.dir.join(format!("trends_{}.json", woeid));
        self.write_cached(&path, data)
    }

    // --- Followers/Following ---
    pub fn get_user_list<T: DeserializeOwned>(
        &self,
        username: &str,
        list_type: &str,
    ) -> Option<CacheHit<T>> {
        if !self.read_enabled {
            return None;
        }
        let path = self
            .dir
            .join(format!("{}_{}.json", list_type, username.to_lowercase()));
        self.read_cached(&path, TTL_FOLLOWERS)
    }

    pub fn set_user_list(
        &self,
        username: &str,
        list_type: &str,
        data: &impl Serialize,
    ) -> Result<(), TwitterApiError> {
        let path = self
            .dir
            .join(format!("{}_{}.json", list_type, username.to_lowercase()));
        self.write_cached(&path, data)
    }

    pub fn search_cache_key(query: &str, query_type: &str, limit: usize) -> String {
        let mut hasher = Sha256::new();
        hasher.update(query.as_bytes());
        hasher.update(b"\0");
        hasher.update(query_type.as_bytes());
        hasher.update(b"\0");
        hasher.update(limit.to_string().as_bytes());
        let result = hasher.finalize();
        hex::encode(&result[..8])
    }

    fn read_cached<T: DeserializeOwned>(&self, path: &Path, ttl: Duration) -> Option<CacheHit<T>> {
        let metadata = std::fs::metadata(path).ok()?;
        let modified = metadata.modified().ok()?;
        let age = SystemTime::now().duration_since(modified).ok()?;
        if age > ttl {
            tracing::debug!("Cache expired for {}", path.display());
            return None;
        }
        let content = std::fs::read_to_string(path).ok()?;
        match serde_json::from_str(&content) {
            Ok(data) => {
                tracing::info!("Cache hit for {}", path.display());
                Some(CacheHit {
                    data,
                    cached_at: modified,
                })
            }
            Err(e) => {
                tracing::warn!("Cache parse error for {}: {}", path.display(), e);
                None
            }
        }
    }

    fn write_cached<T: Serialize>(&self, path: &Path, data: &T) -> Result<(), TwitterApiError> {
        std::fs::create_dir_all(&self.dir)
            .map_err(|e| TwitterApiError::Cache(format!("Failed to create cache dir: {}", e)))?;
        let content = serde_json::to_string_pretty(data)?;
        std::fs::write(path, content)
            .map_err(|e| TwitterApiError::Cache(format!("Failed to write cache: {}", e)))?;
        tracing::debug!("Cached to {}", path.display());
        Ok(())
    }
}
