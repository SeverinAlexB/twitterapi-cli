mod api;
mod cache;
mod cli;
mod config;
mod error;
mod model;
mod output;

use anyhow::{Context, Result};
use clap::Parser;
use cli::{Cli, Commands, UserSection};
use config::AppConfig;
use std::time::SystemTime;

use crate::api::TwitterApiClient;
use crate::cache::Cache;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("twitterapi_cli=warn")),
        )
        .with_target(false)
        .init();

    let config = AppConfig::load(cli.api_key, cli.no_cache, cli.json);

    let api_key = config
        .api_key
        .clone()
        .context("No API key found. Set TWITTERAPI_API_KEY env var, use --api-key, or add api_key to ~/.config/twitterapi-cli/config.toml")?;

    ctrlc::set_handler(|| {
        eprintln!("\nInterrupted.");
        std::process::exit(130);
    })
    .context("Failed to set Ctrl+C handler")?;

    let client = TwitterApiClient::new(api_key);

    match cli.command {
        Commands::Search {
            query,
            limit,
            query_type,
        } => {
            cmd_search(&config, &client, &query, limit, query_type).await?;
        }
        Commands::User {
            username,
            section,
            limit,
        } => {
            cmd_user(&config, &client, &username, section, limit).await?;
        }
        Commands::Tweet { id } => {
            cmd_tweet(&config, &client, &id).await?;
        }
        Commands::Trends { woeid, count } => {
            cmd_trends(&config, &client, woeid, count).await?;
        }
    }

    Ok(())
}

async fn cmd_search(
    config: &AppConfig,
    client: &TwitterApiClient,
    query: &str,
    limit: usize,
    query_type: cli::QueryType,
) -> Result<()> {
    if query.trim().is_empty() {
        anyhow::bail!("Search query cannot be empty");
    }

    let cache = Cache::new(config.cache_dir.clone(), config.no_cache);
    let cache_key = Cache::search_cache_key(query, query_type.as_api_param(), limit);

    // Check cache
    if let Some(hit) = cache.get_search::<model::SearchResult>(&cache_key) {
        if config.json_output {
            println!("{}", serde_json::to_string_pretty(&hit.data)?);
        } else {
            let header = format!("Search: \"{}\" ({} results)", query, hit.data.tweets.len());
            print!("{}", output::format_tweet_list(&hit.data.tweets, &header));
            println!("*Data from: {}*", output::format_cached_at(hit.cached_at));
        }
        return Ok(());
    }

    tracing::info!("Searching tweets...");
    let tweets = client
        .search_tweets(query, query_type.as_api_param(), limit)
        .await?;

    if tweets.is_empty() {
        println!("No results found for: {}", query);
        return Ok(());
    }

    let search_result = model::SearchResult {
        query: query.to_string(),
        query_type: query_type.as_api_param().to_string(),
        tweets: tweets.clone(),
    };

    if let Err(e) = cache.set_search(&cache_key, &search_result) {
        tracing::debug!("Failed to cache search results: {}", e);
    }

    if config.json_output {
        println!("{}", serde_json::to_string_pretty(&search_result)?);
    } else {
        let header = format!("Search: \"{}\" ({} results)", query, tweets.len());
        print!("{}", output::format_tweet_list(&tweets, &header));
        println!(
            "*Data from: {}*",
            output::format_cached_at(SystemTime::now())
        );
    }

    Ok(())
}

async fn cmd_user(
    config: &AppConfig,
    client: &TwitterApiClient,
    username: &str,
    section: UserSection,
    limit: usize,
) -> Result<()> {
    if username.trim().is_empty() {
        anyhow::bail!("Username cannot be empty");
    }

    let cache = Cache::new(config.cache_dir.clone(), config.no_cache);

    match section {
        UserSection::Overview => {
            // Check cache
            if let Some(hit) = cache.get_user(username) {
                let user: model::User = serde_json::from_value(hit.data)?;
                if config.json_output {
                    println!("{}", serde_json::to_string_pretty(&user)?);
                } else {
                    print!("{}", output::format_user_profile(&user));
                    println!("*Data from: {}*", output::format_cached_at(hit.cached_at));
                }
                return Ok(());
            }

            tracing::info!("Fetching user profile...");
            let user = client.get_user_info(username).await?;

            if let Err(e) = cache.set_user(username, &user) {
                tracing::debug!("Failed to cache user: {}", e);
            }

            if config.json_output {
                println!("{}", serde_json::to_string_pretty(&user)?);
            } else {
                print!("{}", output::format_user_profile(&user));
                println!(
                    "*Data from: {}*",
                    output::format_cached_at(SystemTime::now())
                );
            }
        }
        UserSection::Tweets => {
            let cache_key = Cache::search_cache_key(&format!("from:{}", username), "Latest", limit);

            if let Some(hit) = cache.get_search::<model::SearchResult>(&cache_key) {
                if config.json_output {
                    println!("{}", serde_json::to_string_pretty(&hit.data)?);
                } else {
                    let header = format!("Tweets by @{}", username);
                    print!("{}", output::format_tweet_list(&hit.data.tweets, &header));
                    println!("*Data from: {}*", output::format_cached_at(hit.cached_at));
                }
                return Ok(());
            }

            tracing::info!("Fetching user tweets...");
            let query = format!("from:{}", username);
            let tweets = client.search_tweets(&query, "Latest", limit).await?;

            if tweets.is_empty() {
                println!("No tweets found for @{}", username);
                return Ok(());
            }

            let search_result = model::SearchResult {
                query: query.clone(),
                query_type: "Latest".to_string(),
                tweets: tweets.clone(),
            };

            if let Err(e) = cache.set_search(&cache_key, &search_result) {
                tracing::debug!("Failed to cache user tweets: {}", e);
            }

            if config.json_output {
                println!("{}", serde_json::to_string_pretty(&tweets)?);
            } else {
                let header = format!("Tweets by @{} ({} shown)", username, tweets.len());
                print!("{}", output::format_tweet_list(&tweets, &header));
                println!(
                    "*Data from: {}*",
                    output::format_cached_at(SystemTime::now())
                );
            }
        }
        UserSection::Followers => {
            if let Some(hit) = cache.get_user_list::<model::UserList>(username, "followers") {
                if config.json_output {
                    println!("{}", serde_json::to_string_pretty(&hit.data)?);
                } else {
                    let label = format!("Followers of @{}", username);
                    print!("{}", output::format_user_list(&hit.data.users, &label));
                    println!("*Data from: {}*", output::format_cached_at(hit.cached_at));
                }
                return Ok(());
            }

            tracing::info!("Fetching followers...");
            let user_list = client.get_user_followers(username, limit).await?;

            if let Err(e) = cache.set_user_list(username, "followers", &user_list) {
                tracing::debug!("Failed to cache followers: {}", e);
            }

            if config.json_output {
                println!("{}", serde_json::to_string_pretty(&user_list)?);
            } else {
                let label = format!("Followers of @{}", username);
                print!("{}", output::format_user_list(&user_list.users, &label));
                println!(
                    "*Data from: {}*",
                    output::format_cached_at(SystemTime::now())
                );
            }
        }
        UserSection::Following => {
            if let Some(hit) = cache.get_user_list::<model::UserList>(username, "following") {
                if config.json_output {
                    println!("{}", serde_json::to_string_pretty(&hit.data)?);
                } else {
                    let label = format!("@{} is following", username);
                    print!("{}", output::format_user_list(&hit.data.users, &label));
                    println!("*Data from: {}*", output::format_cached_at(hit.cached_at));
                }
                return Ok(());
            }

            tracing::info!("Fetching following...");
            let user_list = client.get_user_followings(username, limit).await?;

            if let Err(e) = cache.set_user_list(username, "following", &user_list) {
                tracing::debug!("Failed to cache following: {}", e);
            }

            if config.json_output {
                println!("{}", serde_json::to_string_pretty(&user_list)?);
            } else {
                let label = format!("@{} is following", username);
                print!("{}", output::format_user_list(&user_list.users, &label));
                println!(
                    "*Data from: {}*",
                    output::format_cached_at(SystemTime::now())
                );
            }
        }
    }

    Ok(())
}

async fn cmd_tweet(config: &AppConfig, client: &TwitterApiClient, ids: &[String]) -> Result<()> {
    let cache = Cache::new(config.cache_dir.clone(), config.no_cache);

    let mut tweets: Vec<model::Tweet> = Vec::new();
    let mut missing_ids: Vec<String> = Vec::new();

    for id in ids {
        if let Some(hit) = cache.get_tweet::<model::Tweet>(id) {
            tweets.push(hit.data);
        } else {
            missing_ids.push(id.clone());
        }
    }

    if !missing_ids.is_empty() {
        tracing::info!("Fetching {} tweet(s)...", missing_ids.len());
        let fetched = client.get_tweets_by_ids(&missing_ids).await?;

        for tweet in &fetched {
            if let Err(e) = cache.set_tweet(&tweet.id, tweet) {
                tracing::debug!("Failed to cache tweet {}: {}", tweet.id, e);
            }
        }

        tweets.extend(fetched);
    }

    // Sort tweets to match input order
    let ordered: Vec<&model::Tweet> = ids
        .iter()
        .filter_map(|id| tweets.iter().find(|t| t.id == *id))
        .collect();

    if ordered.is_empty() {
        println!("No tweets found for the given IDs.");
        return Ok(());
    }

    if config.json_output {
        println!("{}", serde_json::to_string_pretty(&ordered)?);
        return Ok(());
    }

    for tweet in &ordered {
        print!("{}", output::format_tweet(tweet, None));
    }

    println!(
        "*Data from: {}*",
        output::format_cached_at(SystemTime::now())
    );

    Ok(())
}

async fn cmd_trends(
    config: &AppConfig,
    client: &TwitterApiClient,
    woeid: u64,
    count: usize,
) -> Result<()> {
    let cache = Cache::new(config.cache_dir.clone(), config.no_cache);

    if let Some(hit) = cache.get_trends::<Vec<model::Trend>>(woeid) {
        let trends = &hit.data[..hit.data.len().min(count)];
        if config.json_output {
            println!("{}", serde_json::to_string_pretty(&trends)?);
        } else {
            print!("{}", output::format_trends(trends));
            println!("*Data from: {}*", output::format_cached_at(hit.cached_at));
        }
        return Ok(());
    }

    tracing::info!("Fetching trends...");
    let trends = client.get_trends(woeid, count).await?;

    if let Err(e) = cache.set_trends(woeid, &trends) {
        tracing::debug!("Failed to cache trends: {}", e);
    }

    if config.json_output {
        println!("{}", serde_json::to_string_pretty(&trends)?);
    } else {
        print!("{}", output::format_trends(&trends));
        println!(
            "*Data from: {}*",
            output::format_cached_at(SystemTime::now())
        );
    }

    Ok(())
}
