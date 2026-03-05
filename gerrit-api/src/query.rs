/// Builder for constructing Gerrit change query strings.
///
/// Gerrit search uses operators like `status:open`, `owner:self`, `project:foo`.
/// This builder provides a typed interface for constructing these queries.
#[derive(Debug, Clone, Default)]
pub struct QueryBuilder {
    parts: Vec<String>,
}

impl QueryBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn status(mut self, status: &str) -> Self {
        self.parts.push(format!("status:{status}"));
        self
    }

    pub fn owner(mut self, owner: &str) -> Self {
        self.parts.push(format!("owner:{owner}"));
        self
    }

    pub fn project(mut self, project: &str) -> Self {
        self.parts.push(format!("project:{project}"));
        self
    }

    pub fn branch(mut self, branch: &str) -> Self {
        self.parts.push(format!("branch:{branch}"));
        self
    }

    pub fn topic(mut self, topic: &str) -> Self {
        self.parts.push(format!("topic:{topic}"));
        self
    }

    pub fn reviewer(mut self, reviewer: &str) -> Self {
        self.parts.push(format!("reviewer:{reviewer}"));
        self
    }

    pub fn is(mut self, predicate: &str) -> Self {
        self.parts.push(format!("is:{predicate}"));
        self
    }

    pub fn change(mut self, change: &str) -> Self {
        self.parts.push(change.to_string());
        self
    }

    /// Add a raw query fragment (e.g. for operators not covered by named methods).
    pub fn raw(mut self, fragment: &str) -> Self {
        self.parts.push(fragment.to_string());
        self
    }

    pub fn build(self) -> String {
        self.parts.join("+")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_query() {
        let q = QueryBuilder::new()
            .status("open")
            .project("myproject")
            .build();
        assert_eq!(q, "status:open+project:myproject");
    }

    #[test]
    fn test_empty_query() {
        let q = QueryBuilder::new().build();
        assert_eq!(q, "");
    }

    #[test]
    fn test_raw_query() {
        let q = QueryBuilder::new()
            .status("open")
            .raw("label:Code-Review+2")
            .build();
        assert_eq!(q, "status:open+label:Code-Review+2");
    }
}
