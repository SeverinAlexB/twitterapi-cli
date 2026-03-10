use serde::{Deserialize, Serialize};

/// Generic API response wrapper from twitterapi.io
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ApiResponse<T> {
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub msg: Option<String>,
    pub data: Option<T>,
    #[serde(default)]
    pub has_next_page: bool,
    #[serde(default)]
    pub next_cursor: Option<String>,
}

/// Twitter user profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(default, alias = "id")]
    pub id: String,
    #[serde(default, alias = "userName")]
    pub user_name: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default, alias = "followers", alias = "followersCount")]
    pub followers_count: u64,
    #[serde(default, alias = "following", alias = "followingCount")]
    pub following_count: u64,
    #[serde(default, alias = "statusesCount", alias = "tweetsCount")]
    pub tweets_count: u64,
    #[serde(default, alias = "listedCount")]
    pub listed_count: u64,
    #[serde(default, alias = "createdAt")]
    pub created_at: Option<String>,
    #[serde(default)]
    pub location: Option<String>,
    #[serde(default, alias = "isVerified")]
    pub is_verified: bool,
    #[serde(default, alias = "isBlueVerified")]
    pub is_blue_verified: bool,
    #[serde(default, alias = "profileImageUrl", alias = "profilePicture")]
    pub profile_image_url: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
}

/// Twitter tweet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tweet {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub text: String,
    #[serde(default, alias = "createdAt")]
    pub created_at: Option<String>,
    #[serde(default)]
    pub author: Option<TweetAuthor>,
    #[serde(default, alias = "retweetCount")]
    pub retweet_count: u64,
    #[serde(default, alias = "likeCount")]
    pub like_count: u64,
    #[serde(default, alias = "replyCount")]
    pub reply_count: u64,
    #[serde(default, alias = "quoteCount")]
    pub quote_count: u64,
    #[serde(default, alias = "viewCount")]
    pub view_count: u64,
    #[serde(default, alias = "bookmarkCount")]
    pub bookmark_count: u64,
    #[serde(default, alias = "conversationId")]
    pub conversation_id: Option<String>,
    #[serde(default)]
    pub entities: Option<serde_json::Value>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default, alias = "isRetweet")]
    pub is_retweet: bool,
    #[serde(default, alias = "isReply")]
    pub is_reply: bool,
    #[serde(default, alias = "isQuote")]
    pub is_quote: bool,
}

/// Nested author info within a tweet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TweetAuthor {
    #[serde(default, alias = "userName")]
    pub user_name: String,
    #[serde(default)]
    pub name: String,
    #[serde(default, alias = "profileImageUrl")]
    pub profile_image_url: Option<String>,
    #[serde(default, alias = "isVerified")]
    pub is_verified: bool,
    #[serde(default, alias = "isBlueVerified")]
    pub is_blue_verified: bool,
}

/// Trending topic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trend {
    #[serde(default)]
    pub name: String,
    #[serde(default, alias = "tweetCount")]
    pub tweet_count: u64,
    #[serde(default)]
    pub query: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
}

/// Wrapper for search results (for caching)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub query: String,
    pub query_type: String,
    pub tweets: Vec<Tweet>,
}

/// Wrapper for user followers/following lists (for caching)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserList {
    pub users: Vec<User>,
}
