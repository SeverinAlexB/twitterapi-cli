use crate::error::TwitterApiError;
use crate::model::Tweet;

use super::TwitterApiClient;

/// Response wrapper for tweet endpoints that use `"tweets"` key
#[derive(serde::Deserialize)]
struct TweetListResponse {
    #[serde(default)]
    tweets: Vec<Tweet>,
    #[serde(default)]
    has_next_page: bool,
    #[serde(default)]
    next_cursor: Option<String>,
}

impl TwitterApiClient {
    /// GET /twitter/tweets?tweet_ids=<id1>,<id2>,...
    pub async fn get_tweets_by_ids(&self, ids: &[String]) -> Result<Vec<Tweet>, TwitterApiError> {
        let ids_str = ids.join(",");
        let params = vec![("tweet_ids", ids_str)];
        let text = self.get_with_retry("/twitter/tweets", &params).await?;
        let resp: TweetListResponse = serde_json::from_str(&text)?;
        Ok(resp.tweets)
    }

    /// GET /twitter/tweet/advanced_search with cursor-based pagination
    pub async fn search_tweets(
        &self,
        query: &str,
        query_type: &str,
        limit: usize,
    ) -> Result<Vec<Tweet>, TwitterApiError> {
        let mut all_tweets: Vec<Tweet> = Vec::new();
        let mut cursor = String::new();

        loop {
            if all_tweets.len() >= limit {
                break;
            }

            let mut params = vec![
                ("query", query.to_string()),
                ("queryType", query_type.to_string()),
                ("count", limit.to_string()),
            ];
            if !cursor.is_empty() {
                params.push(("cursor", cursor.clone()));
            }

            let text = self
                .get_with_retry("/twitter/tweet/advanced_search", &params)
                .await?;
            let resp: TweetListResponse = serde_json::from_str(&text)?;

            if resp.tweets.is_empty() {
                break;
            }
            all_tweets.extend(resp.tweets);

            if resp.has_next_page {
                if let Some(next) = resp.next_cursor {
                    cursor = next;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        all_tweets.truncate(limit);
        Ok(all_tweets)
    }
}
