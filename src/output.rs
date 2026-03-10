use crate::model::{Trend, Tweet, User};
use std::time::SystemTime;

pub fn format_user_profile(user: &User) -> String {
    let mut out = String::new();

    let verified = if user.is_blue_verified || user.is_verified {
        " ✓"
    } else {
        ""
    };

    out.push_str(&format!(
        "# {} (@{}){}\n\n",
        user.name, user.user_name, verified
    ));

    if let Some(ref desc) = user.description {
        if !desc.is_empty() {
            out.push_str(&format!("{}\n\n", desc));
        }
    }

    if let Some(ref location) = user.location {
        if !location.is_empty() {
            out.push_str(&format!("**Location:** {}\n", location));
        }
    }

    if let Some(ref url) = user.url {
        if !url.is_empty() {
            out.push_str(&format!("**URL:** {}\n", url));
        }
    }

    if let Some(ref created) = user.created_at {
        out.push_str(&format!("**Joined:** {}\n", created));
    }

    out.push('\n');
    out.push_str(&format!(
        "| Followers | Following | Tweets |\n|---|---|---|\n| {} | {} | {} |\n",
        format_number(user.followers_count),
        format_number(user.following_count),
        format_number(user.tweets_count),
    ));

    out.push('\n');
    out
}

pub fn format_tweet(tweet: &Tweet, index: Option<usize>) -> String {
    let mut out = String::new();

    let prefix = match index {
        Some(i) => format!("{}. ", i),
        None => String::new(),
    };

    let author_name = tweet
        .author
        .as_ref()
        .map(|a| a.name.as_str())
        .unwrap_or("Unknown");
    let author_username = tweet
        .author
        .as_ref()
        .map(|a| a.user_name.as_str())
        .unwrap_or("unknown");

    out.push_str(&format!(
        "{}**{}** (@{})\n",
        prefix, author_name, author_username
    ));

    out.push_str(&format!("{}\n", tweet.text));

    // Engagement metrics
    let mut metrics = Vec::new();
    if tweet.like_count > 0 {
        metrics.push(format!("♥ {}", format_number(tweet.like_count)));
    }
    if tweet.retweet_count > 0 {
        metrics.push(format!("↻ {}", format_number(tweet.retweet_count)));
    }
    if tweet.reply_count > 0 {
        metrics.push(format!("↩ {}", format_number(tweet.reply_count)));
    }
    if tweet.view_count > 0 {
        metrics.push(format!("👁 {}", format_number(tweet.view_count)));
    }
    if !metrics.is_empty() {
        out.push_str(&format!("{}\n", metrics.join(" | ")));
    }

    // Tweet URL
    if let Some(ref url) = tweet.url {
        out.push_str(&format!("{}\n", url));
    } else {
        out.push_str(&format!(
            "https://x.com/{}/status/{}\n",
            author_username, tweet.id
        ));
    }

    if let Some(ref date) = tweet.created_at {
        out.push_str(&format!("*{}*\n", date));
    }

    out.push('\n');
    out
}

pub fn format_tweet_list(tweets: &[Tweet], header: &str) -> String {
    let mut out = String::new();
    out.push_str(&format!("## {}\n\n", header));

    for (i, tweet) in tweets.iter().enumerate() {
        out.push_str(&format_tweet(tweet, Some(i + 1)));
    }

    out
}

pub fn format_user_list(users: &[User], label: &str) -> String {
    let mut out = String::new();
    out.push_str(&format!("## {} ({} shown)\n\n", label, users.len()));

    for (i, user) in users.iter().enumerate() {
        let verified = if user.is_blue_verified || user.is_verified {
            " ✓"
        } else {
            ""
        };
        out.push_str(&format!(
            "{}. **{}** (@{}){} — {} followers\n",
            i + 1,
            user.name,
            user.user_name,
            verified,
            format_number(user.followers_count),
        ));
        if let Some(ref desc) = user.description {
            if !desc.is_empty() {
                let short = if desc.len() > 120 {
                    format!("{}...", &desc[..120])
                } else {
                    desc.clone()
                };
                out.push_str(&format!("   {}\n", short));
            }
        }
    }

    out.push('\n');
    out
}

pub fn format_trends(trends: &[Trend]) -> String {
    let mut out = String::new();
    out.push_str("## Trending Topics\n\n");

    for (i, trend) in trends.iter().enumerate() {
        let count = if trend.tweet_count > 0 {
            format!(" ({} tweets)", format_number(trend.tweet_count))
        } else {
            String::new()
        };
        out.push_str(&format!("{}. **{}**{}\n", i + 1, trend.name, count));
    }

    out.push('\n');
    out
}

pub fn format_cached_at(cached_at: SystemTime) -> String {
    let duration = cached_at
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = duration.as_secs() as i64;

    let days = secs / 86400;
    let remaining = secs % 86400;
    let hours = remaining / 3600;
    let minutes = (remaining % 3600) / 60;

    let mut y = 1970i64;
    let mut d = days;
    loop {
        let days_in_year = if y % 4 == 0 && (y % 100 != 0 || y % 400 == 0) {
            366
        } else {
            365
        };
        if d < days_in_year {
            break;
        }
        d -= days_in_year;
        y += 1;
    }
    let leap = y % 4 == 0 && (y % 100 != 0 || y % 400 == 0);
    let month_days = [
        31,
        if leap { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];
    let mut m = 0usize;
    for (i, &md) in month_days.iter().enumerate() {
        if d < md {
            m = i;
            break;
        }
        d -= md;
    }

    format!(
        "{:04}-{:02}-{:02} {:02}:{:02} UTC",
        y,
        m + 1,
        d + 1,
        hours,
        minutes
    )
}

pub fn format_number(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, ch) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(ch);
    }
    result.chars().rev().collect()
}
