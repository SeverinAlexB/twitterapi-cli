use crate::error::TwitterApiError;
use crate::model::{User, UserList};

use super::TwitterApiClient;

/// Response for followers endpoint: `{ "followers": [...], "next_cursor": "..." }`
#[derive(serde::Deserialize)]
struct FollowersResponse {
    #[serde(default)]
    followers: Vec<User>,
    #[serde(default)]
    has_next_page: bool,
    #[serde(default)]
    next_cursor: Option<String>,
}

/// Response for followings endpoint: `{ "followings": [...], "next_cursor": "..." }`
#[derive(serde::Deserialize)]
struct FollowingsResponse {
    #[serde(default)]
    followings: Vec<User>,
    #[serde(default)]
    has_next_page: bool,
    #[serde(default)]
    next_cursor: Option<String>,
}

impl TwitterApiClient {
    /// GET /twitter/user/info?userName=<username>
    pub async fn get_user_info(&self, username: &str) -> Result<User, TwitterApiError> {
        let params = vec![("userName", username.to_string())];
        let resp = self.get_json::<User>("/twitter/user/info", &params).await?;
        resp.data
            .ok_or_else(|| TwitterApiError::Api(format!("User '{}' not found", username)))
    }

    /// GET /twitter/user/followers with cursor-based pagination
    pub async fn get_user_followers(
        &self,
        username: &str,
        limit: usize,
    ) -> Result<UserList, TwitterApiError> {
        let mut all_users: Vec<User> = Vec::new();
        let mut cursor = String::new();

        loop {
            if all_users.len() >= limit {
                break;
            }

            let mut params = vec![("userName", username.to_string())];
            if !cursor.is_empty() {
                params.push(("cursor", cursor.clone()));
            }

            let text = self
                .get_with_retry("/twitter/user/followers", &params)
                .await?;
            let resp: FollowersResponse = serde_json::from_str(&text)?;

            if resp.followers.is_empty() {
                break;
            }
            all_users.extend(resp.followers);

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

        all_users.truncate(limit);
        Ok(UserList { users: all_users })
    }

    /// GET /twitter/user/followings with cursor-based pagination
    pub async fn get_user_followings(
        &self,
        username: &str,
        limit: usize,
    ) -> Result<UserList, TwitterApiError> {
        let mut all_users: Vec<User> = Vec::new();
        let mut cursor = String::new();

        loop {
            if all_users.len() >= limit {
                break;
            }

            let mut params = vec![("userName", username.to_string())];
            if !cursor.is_empty() {
                params.push(("cursor", cursor.clone()));
            }

            let text = self
                .get_with_retry("/twitter/user/followings", &params)
                .await?;
            let resp: FollowingsResponse = serde_json::from_str(&text)?;

            if resp.followings.is_empty() {
                break;
            }
            all_users.extend(resp.followings);

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

        all_users.truncate(limit);
        Ok(UserList { users: all_users })
    }
}
