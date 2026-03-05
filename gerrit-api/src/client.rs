use base64::Engine;
use reqwest::blocking::Client;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use url::Url;

use crate::error::{GerritError, Result};

/// A client for the Gerrit REST API.
///
/// All authenticated endpoints use HTTP Basic Auth and the `/a/` path prefix.
/// Gerrit responses are prefixed with `)]}'` which is stripped before JSON parsing.
#[derive(Debug, Clone)]
pub struct GerritClient {
    base_url: Url,
    username: String,
    password: String,
    client: Client,
}

const GERRIT_MAGIC_PREFIX: &str = ")]}'";

impl GerritClient {
    /// Create a new GerritClient.
    ///
    /// `base_url` should be the root URL of the Gerrit instance (e.g. `https://review.example.com`).
    /// The trailing slash is normalized automatically.
    pub fn new(base_url: &str, username: &str, password: &str) -> Result<Self> {
        let mut url = Url::parse(base_url)?;
        // Ensure the base URL has a trailing slash for proper joining
        if !url.path().ends_with('/') {
            url.set_path(&format!("{}/", url.path()));
        }

        let client = Client::builder().user_agent("gerrit-cli/0.1.0").build()?;

        Ok(Self {
            base_url: url,
            username: username.to_string(),
            password: password.to_string(),
            client,
        })
    }

    /// Create a client for unauthenticated access (read-only).
    pub fn anonymous(base_url: &str) -> Result<Self> {
        Self::new(base_url, "", "")
    }

    fn auth_header(&self) -> String {
        let credentials = format!("{}:{}", self.username, self.password);
        let encoded = base64::engine::general_purpose::STANDARD.encode(credentials.as_bytes());
        format!("Basic {encoded}")
    }

    fn is_authenticated(&self) -> bool {
        !self.username.is_empty()
    }

    /// Build a full URL for an API endpoint.
    /// Authenticated requests use the `/a/` prefix.
    fn url(&self, path: &str) -> Result<Url> {
        let path = path.trim_start_matches('/');
        let full_path = if self.is_authenticated() {
            format!("a/{path}")
        } else {
            path.to_string()
        };
        Ok(self.base_url.join(&full_path)?)
    }

    /// Strip the Gerrit magic prefix from a response body.
    fn strip_magic(body: &str) -> &str {
        let trimmed = body.trim_start();
        trimmed
            .strip_prefix(GERRIT_MAGIC_PREFIX)
            .unwrap_or(trimmed)
            .trim_start()
    }

    /// Perform an authenticated GET request and return the parsed JSON response.
    pub fn get<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = self.url(path)?;
        let mut req = self.client.get(url);

        if self.is_authenticated() {
            req = req.header(AUTHORIZATION, self.auth_header());
        }

        let resp = req.send()?;
        let status = resp.status();
        let body = resp.text()?;

        if !status.is_success() {
            return Err(GerritError::Api {
                status: status.as_u16(),
                message: body,
            });
        }

        let json_str = Self::strip_magic(&body);
        Ok(serde_json::from_str(json_str)?)
    }

    /// Perform an authenticated POST request with a JSON body.
    pub fn post<B: serde::Serialize, T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let url = self.url(path)?;
        let mut req = self
            .client
            .post(url)
            .header(CONTENT_TYPE, "application/json");

        if self.is_authenticated() {
            req = req.header(AUTHORIZATION, self.auth_header());
        }

        let resp = req.json(body).send()?;
        let status = resp.status();
        let body_text = resp.text()?;

        if !status.is_success() {
            return Err(GerritError::Api {
                status: status.as_u16(),
                message: body_text,
            });
        }

        let json_str = Self::strip_magic(&body_text);
        Ok(serde_json::from_str(json_str)?)
    }

    /// Perform an authenticated POST request that returns no meaningful body (e.g. 204).
    pub fn post_no_content<B: serde::Serialize>(&self, path: &str, body: &B) -> Result<()> {
        let url = self.url(path)?;
        let mut req = self
            .client
            .post(url)
            .header(CONTENT_TYPE, "application/json");

        if self.is_authenticated() {
            req = req.header(AUTHORIZATION, self.auth_header());
        }

        let resp = req.json(body).send()?;
        let status = resp.status();

        if !status.is_success() {
            let body_text = resp.text()?;
            return Err(GerritError::Api {
                status: status.as_u16(),
                message: body_text,
            });
        }

        Ok(())
    }

    /// Perform a GET request and return the raw response bytes (for non-JSON endpoints like hooks).
    pub fn get_raw(&self, path: &str) -> Result<Vec<u8>> {
        let url = self.url(path)?;
        let mut req = self.client.get(url);

        if self.is_authenticated() {
            req = req.header(AUTHORIZATION, self.auth_header());
        }

        let resp = req.send()?;
        let status = resp.status();

        if !status.is_success() {
            let body = resp.text()?;
            return Err(GerritError::Api {
                status: status.as_u16(),
                message: body,
            });
        }

        Ok(resp.bytes()?.to_vec())
    }

    pub fn base_url(&self) -> &Url {
        &self.base_url
    }

    pub fn username(&self) -> &str {
        &self.username
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_magic() {
        let body = ")]}'\n{\"key\": \"value\"}";
        assert_eq!(GerritClient::strip_magic(body), "{\"key\": \"value\"}");
    }

    #[test]
    fn test_strip_magic_no_prefix() {
        let body = "{\"key\": \"value\"}";
        assert_eq!(GerritClient::strip_magic(body), "{\"key\": \"value\"}");
    }

    #[test]
    fn test_url_building() {
        let client = GerritClient::new("https://review.example.com", "user", "pass").unwrap();
        let url = client.url("changes/").unwrap();
        assert_eq!(url.as_str(), "https://review.example.com/a/changes/");
    }

    #[test]
    fn test_url_building_anonymous() {
        let client = GerritClient::anonymous("https://review.example.com").unwrap();
        let url = client.url("changes/").unwrap();
        assert_eq!(url.as_str(), "https://review.example.com/changes/");
    }
}
