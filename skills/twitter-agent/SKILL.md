---
name: twitter-agent
description: "Search Twitter/X for tweets, user profiles, and trending topics using twitterapi-cli. Use when the user asks about tweets, Twitter discussions, user profiles, trending topics, or wants to monitor conversations — including searching tweets, analyzing users, comparing influence, finding viral content, or tracking trends. Triggers include: 'what is X tweeting about', 'find tweets about', 'who follows', 'what's trending', 'search Twitter for', 'track this conversation', 'compare these accounts', 'find viral tweets about'."
user_invocable: false
---

# Twitter Agent

This skill provides access to Twitter/X data through the `twitterapi-cli` command-line tool.

## Prerequisites

- `twitterapi-cli` must be installed and on PATH
- `TWITTERAPI_API_KEY` env var must be set (or use `--api-key`)

## Commands

### Search Tweets

```bash
twitterapi-cli search "AI agents" --limit 20 --type latest
twitterapi-cli search "from:elonmusk" --limit 5
twitterapi-cli search "#bitcoin" --type top --limit 10
```

Options:
- `query` (positional, required) — search query
- `--limit <N>` — max results (default 20)
- `--type <latest|top>` — result type (default latest)

### User Profile

```bash
twitterapi-cli user elonmusk
twitterapi-cli user elonmusk --section tweets --limit 10
twitterapi-cli user elonmusk --section followers --limit 50
twitterapi-cli user elonmusk --section following --limit 50
```

Options:
- `username` (positional, required) — Twitter username without @
- `--section <overview|tweets|followers|following>` — data section (default overview)
- `--limit <N>` — max results for tweets/followers/following (default 20)

### Get Tweets by ID

```bash
twitterapi-cli tweet 1234567890
twitterapi-cli tweet 1234567890 9876543210
```

### Trending Topics

```bash
twitterapi-cli trends
twitterapi-cli trends --woeid 23424977  # US trends
twitterapi-cli trends --count 10
```

Options:
- `--woeid <id>` — Where On Earth ID (default 1 = worldwide)
- `--count <N>` — number of trends (default 30)

## Global Flags

- `--api-key <key>` — override env var
- `--no-cache` — bypass cache
- `--json` — raw JSON output (pipe to `jq` for processing)

## Workflows

### Research a topic
1. `twitterapi-cli search "topic" --limit 20 --type top` — find popular tweets
2. `twitterapi-cli user <username>` — check influential authors
3. `twitterapi-cli user <username> --section tweets --limit 10` — read their recent tweets

### Monitor trends
1. `twitterapi-cli trends` — see what's trending worldwide
2. `twitterapi-cli search "<trend_name>" --limit 10` — dive into a trend

### Analyze a user
1. `twitterapi-cli user <username>` — profile overview
2. `twitterapi-cli user <username> --section tweets --limit 20` — recent activity
3. `twitterapi-cli user <username> --section followers --limit 50` — who follows them
4. `twitterapi-cli user <username> --section following --limit 50` — who they follow

### Track a conversation thread
1. `twitterapi-cli tweet <tweet_id>` — get the original tweet
2. `twitterapi-cli search "conversation_id:<tweet_id>" --limit 20` — find replies in the thread
3. `twitterapi-cli user <author>` — check the author's profile for context

### Compare user influence
1. `twitterapi-cli user <user1>` — get first user's stats
2. `twitterapi-cli user <user2>` — get second user's stats
3. Compare followers, following, tweet counts, and verification status
4. `twitterapi-cli user <user1> --section tweets --limit 10` — compare recent engagement

### Find viral content on a topic
1. `twitterapi-cli search "topic" --type top --limit 20` — find most popular tweets
2. Look for tweets with high engagement (♥, ↻, 👁 metrics)
3. `twitterapi-cli user <author>` — check if the author is an influencer
4. `twitterapi-cli tweet <tweet_id>` — get full details on viral tweets

## Twitter Search Tips

The search query supports Twitter's advanced search operators:

| Operator | Description | Example |
|---|---|---|
| `from:user` | Tweets from a specific user | `from:elonmusk` |
| `to:user` | Replies to a specific user | `to:elonmusk` |
| `#hashtag` | Tweets with a hashtag | `#bitcoin` |
| `@mention` | Tweets mentioning a user | `@github` |
| `since:YYYY-MM-DD` | Tweets after a date | `since:2026-01-01` |
| `until:YYYY-MM-DD` | Tweets before a date | `until:2026-03-01` |
| `min_faves:N` | Minimum likes | `min_faves:1000` |
| `min_retweets:N` | Minimum retweets | `min_retweets:500` |
| `filter:links` | Only tweets with links | `AI filter:links` |
| `filter:images` | Only tweets with images | `sunset filter:images` |
| `OR` | Match either term | `bitcoin OR ethereum` |
| `-` | Exclude a term | `AI -crypto` |

Operators can be combined: `from:elonmusk AI agents since:2026-01-01 min_faves:100`

## Tips

- Use `--type top` to find popular/viral tweets; use `--type latest` (default) for recency
- Use `--json` output with `jq` for programmatic processing of results
- Use `--no-cache` when you need real-time data (e.g., trending topics, breaking news)
- Cache TTLs: user profiles 1 day, tweets 7 days, search 1 hour, trends 5 minutes, followers 1 hour
- Tweet IDs from search results can be passed to `tweet` for full details
- Combine `from:user` with topic keywords to find what someone said about a specific subject
- For large follower/following lists, increase `--limit` (default 20) up to 200
