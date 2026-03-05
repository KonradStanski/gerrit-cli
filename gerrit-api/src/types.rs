use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccountInfo {
    #[serde(rename = "_account_id")]
    pub account_id: Option<i64>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub username: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GitPersonInfo {
    pub name: String,
    pub email: String,
    pub date: Option<String>,
    pub tz: Option<i32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommitInfo {
    pub commit: Option<String>,
    pub parents: Option<Vec<CommitInfo>>,
    pub author: Option<GitPersonInfo>,
    pub committer: Option<GitPersonInfo>,
    pub subject: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FetchInfo {
    pub url: Option<String>,
    #[serde(rename = "ref")]
    pub fetch_ref: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RevisionInfo {
    #[serde(rename = "_number")]
    pub number: Option<i32>,
    #[serde(rename = "ref")]
    pub revision_ref: Option<String>,
    pub commit: Option<CommitInfo>,
    pub fetch: Option<HashMap<String, FetchInfo>>,
    pub kind: Option<String>,
    pub created: Option<String>,
    pub uploader: Option<AccountInfo>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LabelInfo {
    pub approved: Option<AccountInfo>,
    pub rejected: Option<AccountInfo>,
    pub recommended: Option<AccountInfo>,
    pub disliked: Option<AccountInfo>,
    pub value: Option<i32>,
    pub default_value: Option<i32>,
    pub optional: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChangeMessageInfo {
    pub id: Option<String>,
    pub author: Option<AccountInfo>,
    pub date: Option<String>,
    pub message: Option<String>,
    #[serde(rename = "_revision_number")]
    pub revision_number: Option<i32>,
    pub tag: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChangeInfo {
    pub id: Option<String>,
    pub project: Option<String>,
    pub branch: Option<String>,
    pub topic: Option<String>,
    pub change_id: Option<String>,
    pub subject: Option<String>,
    pub status: Option<String>,
    pub created: Option<String>,
    pub updated: Option<String>,
    pub submitted: Option<String>,
    pub insertions: Option<i32>,
    pub deletions: Option<i32>,
    #[serde(rename = "_number")]
    pub number: Option<i64>,
    pub owner: Option<AccountInfo>,
    pub labels: Option<HashMap<String, LabelInfo>>,
    pub current_revision: Option<String>,
    pub revisions: Option<HashMap<String, RevisionInfo>>,
    pub messages: Option<Vec<ChangeMessageInfo>>,
    #[serde(rename = "_more_changes")]
    pub more_changes: Option<bool>,
    pub mergeable: Option<bool>,
    pub submittable: Option<bool>,
    pub reviewers: Option<HashMap<String, Vec<AccountInfo>>>,
    pub total_comment_count: Option<i32>,
    pub unresolved_comment_count: Option<i32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommentRange {
    pub start_line: i32,
    pub start_character: i32,
    pub end_line: i32,
    pub end_character: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommentInfo {
    pub id: Option<String>,
    pub path: Option<String>,
    pub line: Option<i32>,
    pub range: Option<CommentRange>,
    pub message: Option<String>,
    pub author: Option<AccountInfo>,
    pub updated: Option<String>,
    pub in_reply_to: Option<String>,
    pub unresolved: Option<bool>,
    #[serde(rename = "patch_set")]
    pub patch_set: Option<i32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReviewInput {
    pub message: Option<String>,
    pub labels: Option<HashMap<String, i32>>,
    pub tag: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReviewResult {
    pub labels: Option<HashMap<String, i32>>,
    pub reviewers: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubmitInput {
    pub on_behalf_of: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AbandonInput {
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReviewerInput {
    pub reviewer: String,
}
