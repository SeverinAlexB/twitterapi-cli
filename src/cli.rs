use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(
    name = "twitterapi-cli",
    version,
    about = "Query Twitter/X data from the command line via twitterapi.io"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// API key for twitterapi.io (overrides TWITTERAPI_API_KEY env var)
    #[arg(long, global = true)]
    pub api_key: Option<String>,

    /// Bypass the local cache and fetch fresh data
    #[arg(long, global = true)]
    pub no_cache: bool,

    /// Output raw JSON instead of Markdown
    #[arg(long, global = true)]
    pub json: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Search tweets
    Search {
        /// Search query (e.g., "AI agents", "from:elonmusk")
        query: String,

        /// Max number of results
        #[arg(long, default_value = "20")]
        limit: usize,

        /// Type of search results
        #[arg(long = "type", value_enum, default_value_t = QueryType::Latest)]
        query_type: QueryType,
    },

    /// Get user profile and related data
    User {
        /// Twitter username (without @)
        username: String,

        /// Which data to fetch
        #[arg(long, value_enum, default_value_t = UserSection::Overview)]
        section: UserSection,

        /// Max number of results (for tweets/followers/following)
        #[arg(long, default_value = "20")]
        limit: usize,
    },

    /// Get tweet(s) by ID
    Tweet {
        /// One or more tweet IDs
        #[arg(required = true)]
        id: Vec<String>,
    },

    /// Get trending topics
    Trends {
        /// Where On Earth ID (1 = worldwide)
        #[arg(long, default_value = "1")]
        woeid: u64,

        /// Number of trends to show
        #[arg(long, default_value = "30")]
        count: usize,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum QueryType {
    Latest,
    Top,
}

impl QueryType {
    pub fn as_api_param(self) -> &'static str {
        match self {
            QueryType::Latest => "Latest",
            QueryType::Top => "Top",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum UserSection {
    Overview,
    Tweets,
    Followers,
    Following,
}
