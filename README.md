# twitterapi-cli

A Rust command-line tool for querying [Twitter/X](https://x.com) data. Designed for both AI agents and humans — clean commands, Markdown output, and local caching for fast repeat queries.

Uses the [twitterapi.io](https://twitterapi.io) API directly (no browser needed). An API key is required — [get one here](https://twitterapi.io).

## Installation

```bash
curl -fsSL https://raw.githubusercontent.com/SeverinAlexB/twitterapi-cli/master/install.sh | bash
```

This downloads the latest release binary for your platform and installs it to `/usr/local/bin`. Run the same command again to update.

### Build from source

Requires [Rust](https://www.rust-lang.org/tools/install) (1.70+).

```bash
git clone https://github.com/SeverinAlexB/twitterapi-cli.git
cd twitterapi-cli
cargo build --release
```

The binary will be at `target/release/twitterapi-cli`.

## Usage

```
twitterapi-cli <command> [options] [arguments]
```

### Search tweets

```bash
twitterapi-cli search "AI agents" --limit 20
twitterapi-cli search "from:elonmusk" --limit 5
twitterapi-cli search "#bitcoin" --type top --limit 10
twitterapi-cli search "rust programming" --type latest --limit 15
```

**Options:**

| Flag | Description | Default |
|---|---|---|
| `--limit <n>` | Max results | `20` |
| `--type <type>` | `latest` or `top` | `latest` |

**Example output:**

```markdown
## Twitter Search: "AI agents" (20 results)

1. **Sam Altman** (@sama)
   AI agents are going to change everything about how we work.
   ♥ 12,345 | ↻ 2,456 | ↩ 891 | 👁 1,234,567
   https://x.com/sama/status/1234567890
   *2026-03-09T15:30:00.000Z*

2. **Andrej Karpathy** (@kaborthy)
   The next frontier is agents that can use tools reliably...
   ♥ 8,901 | ↻ 1,234
   https://x.com/kaborthy/status/9876543210
   *2026-03-09T14:00:00.000Z*
```

### Get user profile

```bash
twitterapi-cli user elonmusk
twitterapi-cli user elonmusk --section tweets --limit 10
twitterapi-cli user elonmusk --section followers --limit 50
twitterapi-cli user github --section following --limit 20
```

**Options:**

| Flag | Description | Default |
|---|---|---|
| `--section <name>` | `overview`, `tweets`, `followers`, `following` | `overview` |
| `--limit <n>` | Max results (for tweets/followers/following) | `20` |

**Example output:**

```markdown
# Elon Musk (@elonmusk) ✓

Technoking of Tesla, owner of X, founder of SpaceX & xAI

**Location:** Earth
**URL:** https://x.com
**Joined:** Tue Jun 02 20:12:29 +0000 2009

| Followers | Following | Tweets |
|---|---|---|
| 215,432,100 | 1,234 | 52,345 |
```

### Get tweets by ID

```bash
twitterapi-cli tweet 1234567890
twitterapi-cli tweet 1234567890 9876543210
```

Accepts one or more numeric tweet IDs.

**Example output:**

```markdown
## Tweet Details (1 tweet)

**Elon Musk** (@elonmusk)
The future is bright ☀️
♥ 45,678 | ↻ 5,432 | ↩ 2,100 | 👁 12,345,678
https://x.com/elonmusk/status/1234567890
*2026-03-09T12:00:00.000Z*
```

### Get trending topics

```bash
twitterapi-cli trends
twitterapi-cli trends --woeid 23424977 --count 5    # US trends
```

**Options:**

| Flag | Description | Default |
|---|---|---|
| `--woeid <id>` | Where On Earth ID (1 = worldwide) | `1` |
| `--count <n>` | Number of trends to show | `30` |

**Example output:**

```markdown
## Trending Topics

1. **#Bitcoin** (125,000 tweets)
2. **AI Agents** (89,432 tweets)
3. **SpaceX** (67,890 tweets)
```

### Global flags

| Flag | Description | Default |
|---|---|---|
| `--api-key <key>` | twitterapi.io API key | env `TWITTERAPI_API_KEY` |
| `--no-cache` | Bypass local cache and fetch fresh data | — |
| `--json` | Output raw JSON instead of Markdown | — |

## Configuration

Settings are resolved in order of priority:

1. CLI flags (highest)
2. Environment variables (`TWITTERAPI_API_KEY`)
3. Config file
4. Defaults

### Config file

Location: `~/.config/twitterapi-cli/config.toml`

```toml
api_key = "your-api-key-here"
```

An API key is required. [Get one at twitterapi.io.](https://twitterapi.io)

## Caching

Results are cached locally to speed up repeated queries:

- **Location:** `~/Library/Caches/twitterapi-cli/` (macOS) or `~/.cache/twitterapi-cli/` (Linux)
- **TTLs:**
  - User profiles: 1 day
  - Tweets: 7 days
  - Search results: 1 hour
  - Trends: 5 minutes
  - Followers/following: 1 hour

Every result includes a `Data from:` timestamp so you know how fresh the data is. Use `--no-cache` to bypass the cache and fetch fresh data.

## Claude Code skill

This repo includes a [Claude Code skill](https://code.claude.com/docs/en/skills) that teaches AI agents how to use `twitterapi-cli` for Twitter research. With the skill installed, Claude can autonomously search for tweets, analyze user profiles, track trends, and monitor conversations.

### Install the skill

```bash
/install-plugin twitter-agent@SeverinAlexB/twitterapi-cli
```

### What the agent can do

Once installed, Claude can handle requests like:

- *"What is Elon Musk tweeting about today?"*
- *"Find the most popular tweets about AI agents"*
- *"Who are the top followers of @github?"*
- *"What's trending on Twitter right now?"*
- *"Track the conversation around this tweet"*

The skill guides Claude through multi-step workflows — searching, analyzing profiles, tracking trends, and comparing user influence.

### Requirements

The `twitterapi-cli` binary must be available on `PATH`. Build it first:

```bash
cargo build --release
export PATH="$PATH:$(pwd)/target/release"
```
