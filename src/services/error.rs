// src/services/error.rs

//! Strongly-typed error domain for service layer failures.

use std::fmt;

/// Comprehensive error type for API service operations.
///
/// Replaces string-based error propagation with structured variants
/// that enable type-safe error handling in command handlers.
#[derive(Debug)]
pub enum ServiceError {
    /// Network transport failure (connection, timeout, DNS).
    Network { url: String, source: String },

    /// JSON deserialization failure.
    Parse { url: String, source: String },

    /// Resource not found (404-equivalent).
    NotFound { resource: String, identifier: String },

    /// Missing required API credentials.
    MissingToken { token_name: String },

    /// Upstream API returned an error response.
    ApiError { url: String, message: String },

    /// HTTP status code outside 2xx range.
    UnexpectedStatus { url: String, status: u16 },

    /// Expected field missing from API response.
    MissingField { field: String, url: String },
}

impl ServiceError {
    pub fn network(url: &str, err: reqwest::Error) -> Self {
        Self::Network {
            url: url.to_string(),
            source: err.to_string(),
        }
    }

    pub fn parse(url: &str, err: reqwest::Error) -> Self {
        Self::Parse {
            url: url.to_string(),
            source: err.to_string(),
        }
    }

    pub fn not_found(resource: &str, identifier: &str) -> Self {
        Self::NotFound {
            resource: resource.to_string(),
            identifier: identifier.to_string(),
        }
    }

    pub fn missing_token(token_name: &str) -> Self {
        Self::MissingToken {
            token_name: token_name.to_string(),
        }
    }

    pub fn api_error(url: &str, message: Option<&str>) -> Self {
        Self::ApiError {
            url: url.to_string(),
            message: message.unwrap_or("Unknown API error").to_string(),
        }
    }

    pub fn unexpected_status(url: &str, status: reqwest::StatusCode) -> Self {
        Self::UnexpectedStatus {
            url: url.to_string(),
            status: status.as_u16(),
        }
    }

    pub fn missing_field(field: &str, url: &str) -> Self {
        Self::MissingField {
            field: field.to_string(),
            url: url.to_string(),
        }
    }
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Network { url, source } => write!(f, "Network error for {}: {}", url, source),
            Self::Parse { url, source } => write!(f, "Parse error for {}: {}", url, source),
            Self::NotFound { resource, identifier } => {
                write!(f, "{} not found: {}", resource, identifier)
            }
            Self::MissingToken { token_name } => {
                write!(f, "Missing required token: {}", token_name)
            }
            Self::ApiError { url, message } => write!(f, "API error from {}: {}", url, message),
            Self::UnexpectedStatus { url, status } => {
                write!(f, "Unexpected status {} from {}", status, url)
            }
            Self::MissingField { field, url } => {
                write!(f, "Missing field '{}' in response from {}", field, url)
            }
        }
    }
}

impl std::error::Error for ServiceError {}