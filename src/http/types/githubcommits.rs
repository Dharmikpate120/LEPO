use serde::{Deserialize, Serialize};

/// Represents a GitHub commit object.
#[derive(Debug, Serialize, Deserialize)]
pub struct GithubCommits {
    pub sha: String,
    pub node_id: String,
    pub commit: Commit,
    pub url: String,
    pub html_url: String,
    pub comments_url: String,
    pub author: AuthorOrCommitter,
    pub committer: AuthorOrCommitter,
    pub parents: Vec<Parent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Commit {
    pub author: GitUser,
    pub committer: GitUser,
    pub message: String,
    pub tree: Tree,
    pub url: String,
    pub comment_count: u32,
    pub verification: Verification,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitUser {
    pub name: String,
    pub email: String,
    pub date: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tree {
    pub sha: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Verification {
    pub verified: bool,
    pub reason: String,
    pub signature: Option<String>,
    pub payload: Option<String>,
    pub verified_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorOrCommitter {
    pub login: String,
    pub id: u64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub user_type: String,
    pub user_view_type: String,
    pub site_admin: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Parent {
    pub sha: String,
    pub url: String,
    pub html_url: String,
}