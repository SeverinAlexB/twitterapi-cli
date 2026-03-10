use crate::error::TwitterApiError;
use crate::model::Trend;

use super::TwitterApiClient;

/// Raw trend item from API: `{ "trend": { "name": "...", "target": { "query": "..." }, "rank": N } }`
#[derive(serde::Deserialize)]
struct RawTrendWrapper {
    trend: RawTrend,
}

#[derive(serde::Deserialize)]
struct RawTrend {
    #[serde(default)]
    name: String,
    #[serde(default)]
    target: Option<RawTrendTarget>,
    #[serde(default)]
    tweet_count: Option<u64>,
}

#[derive(serde::Deserialize)]
struct RawTrendTarget {
    #[serde(default)]
    query: Option<String>,
}

#[derive(serde::Deserialize)]
struct TrendsResponse {
    #[serde(default)]
    trends: Vec<serde_json::Value>,
}

impl TwitterApiClient {
    /// GET /twitter/trends?woeid=<woeid>&count=<count>
    pub async fn get_trends(
        &self,
        woeid: u64,
        count: usize,
    ) -> Result<Vec<Trend>, TwitterApiError> {
        let params = vec![("woeid", woeid.to_string()), ("count", count.to_string())];
        let text = self.get_with_retry("/twitter/trends", &params).await?;
        tracing::debug!("Trends response: {}", &text[..text.len().min(500)]);
        let resp: TrendsResponse = serde_json::from_str(&text)?;

        let mut trends = Vec::new();
        for item in &resp.trends {
            // Try nested format: { "trend": { "name": "...", ... } }
            if let Ok(wrapper) = serde_json::from_value::<RawTrendWrapper>(item.clone()) {
                let query = wrapper.trend.target.and_then(|t| t.query);
                trends.push(Trend {
                    name: wrapper.trend.name,
                    tweet_count: wrapper.trend.tweet_count.unwrap_or(0),
                    query,
                    url: None,
                });
            }
            // Try flat format: { "name": "...", "tweet_count": N, ... }
            else if let Ok(trend) = serde_json::from_value::<Trend>(item.clone()) {
                trends.push(trend);
            }
        }

        Ok(trends)
    }
}
