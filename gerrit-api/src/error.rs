use thiserror::Error;

#[derive(Error, Debug)]
pub enum GerritError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON parse error: {0}")]
    JsonParse(#[from] serde_json::Error),

    #[error("API error ({status}): {message}")]
    Api { status: u16, message: String },

    #[error("URL parse error: {0}")]
    Url(#[from] url::ParseError),

    #[error("No credentials found for {host}. Set GERRIT_PASSWORD, configure .netrc, or run `gerrit config init`")]
    NoCredentials { host: String },
}

pub type Result<T> = std::result::Result<T, GerritError>;
