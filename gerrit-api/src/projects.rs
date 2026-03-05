use std::collections::HashMap;

use crate::client::GerritClient;
use crate::error::Result;
use crate::types::ProjectInfo;

impl GerritClient {
    /// List projects on the Gerrit server.
    ///
    /// `substring` filters by substring match, `regex` filters by regex.
    /// `limit` caps the number of results returned.
    pub fn list_projects(
        &self,
        substring: Option<&str>,
        regex: Option<&str>,
        limit: Option<u32>,
    ) -> Result<HashMap<String, ProjectInfo>> {
        let mut path = "projects/?d".to_string(); // d = include description
        if let Some(s) = substring {
            path.push_str(&format!("&m={s}"));
        }
        if let Some(r) = regex {
            path.push_str(&format!("&r={r}"));
        }
        if let Some(n) = limit {
            path.push_str(&format!("&n={n}"));
        }
        self.get(&path)
    }

    /// Get info about a single project by name.
    pub fn get_project(&self, name: &str) -> Result<ProjectInfo> {
        let encoded = urlencoding::encode(name);
        let path = format!("projects/{encoded}");
        self.get(&path)
    }
}
