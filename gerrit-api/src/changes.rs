use std::collections::HashMap;

use crate::client::GerritClient;
use crate::error::Result;
use crate::types::*;

impl GerritClient {
    /// Query changes using a Gerrit search query string.
    ///
    /// `query` should be a Gerrit query string (e.g. `status:open+project:foo`).
    /// `opts` are additional query parameters like `n` (limit) and `o` (options).
    pub fn query_changes(
        &self,
        query: &str,
        limit: Option<u32>,
        options: &[&str],
    ) -> Result<Vec<ChangeInfo>> {
        let mut path = format!("changes/?q={query}");
        if let Some(n) = limit {
            path.push_str(&format!("&n={n}"));
        }
        for opt in options {
            path.push_str(&format!("&o={opt}"));
        }
        self.get(&path)
    }

    /// Get a single change by its change ID (number, triplet, or Change-Id).
    pub fn get_change(&self, change_id: &str) -> Result<ChangeInfo> {
        let path = format!("changes/{change_id}");
        self.get(&path)
    }

    /// Get a change with full details (current revision, labels, messages, etc.).
    pub fn get_change_detail(&self, change_id: &str) -> Result<ChangeInfo> {
        let path = format!(
            "changes/{change_id}/detail?o=CURRENT_REVISION&o=CURRENT_COMMIT&o=DOWNLOAD_COMMANDS&o=LABELS&o=DETAILED_LABELS&o=DETAILED_ACCOUNTS&o=MESSAGES"
        );
        self.get(&path)
    }

    /// Get comments on all revisions of a change, grouped by file path.
    pub fn get_comments(&self, change_id: &str) -> Result<HashMap<String, Vec<CommentInfo>>> {
        let path = format!("changes/{change_id}/comments");
        self.get(&path)
    }

    /// Get change messages (the top-level review messages, not inline comments).
    pub fn get_messages(&self, change_id: &str) -> Result<Vec<ChangeMessageInfo>> {
        let path = format!("changes/{change_id}/messages");
        self.get(&path)
    }

    /// Post a review (labels, message) on a change's current revision.
    pub fn set_review(
        &self,
        change_id: &str,
        revision: &str,
        review: &ReviewInput,
    ) -> Result<ReviewResult> {
        let path = format!("changes/{change_id}/revisions/{revision}/review");
        self.post(&path, review)
    }

    /// Submit a change.
    pub fn submit_change(&self, change_id: &str) -> Result<ChangeInfo> {
        let input = SubmitInput { on_behalf_of: None };
        let path = format!("changes/{change_id}/submit");
        self.post(&path, &input)
    }

    /// Abandon a change with an optional message.
    pub fn abandon_change(&self, change_id: &str, message: Option<&str>) -> Result<ChangeInfo> {
        let input = AbandonInput {
            message: message.map(|s| s.to_string()),
        };
        let path = format!("changes/{change_id}/abandon");
        self.post(&path, &input)
    }

    /// Add a reviewer to a change.
    pub fn add_reviewer(&self, change_id: &str, reviewer: &str) -> Result<()> {
        let input = ReviewerInput {
            reviewer: reviewer.to_string(),
        };
        let path = format!("changes/{change_id}/reviewers");
        self.post_no_content(&path, &input)
    }
}
