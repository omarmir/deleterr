use serde::Deserialize;
use std::error::Error;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct DeleterrError {
    details: String,
}

impl DeleterrError {
    pub fn new(msg: &str) -> DeleterrError {
        DeleterrError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for DeleterrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl DeleterrError {
    pub fn add_prefix(mut self, prefix: &str) -> Self {
        self.details = format!("{} {}", prefix, self.details);
        self
    }
}

impl Error for DeleterrError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<reqwest::Error> for DeleterrError {
    fn from(err: reqwest::Error) -> Self {
        DeleterrError::new(err.to_string().as_str())
    }
}

impl From<jammdb::Error> for DeleterrError {
    fn from(err: jammdb::Error) -> Self {
        DeleterrError::new(err.to_string().as_str())
    }
}
