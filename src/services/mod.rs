// src/services/mod.rs (OPTIMIZED)

//! Centralized API service layer for external HTTP data fetching.
//!
//! Provides type-safe, ergonomic interfaces to third-party APIs while abstracting
//! transport concerns from command handlers.

mod error;
mod models;

pub use error::ServiceError;
pub use models::{ExchangerateResponse, JokeResponse, WeatherData};

use reqwest::{Client, StatusCode};
use serde::Deserialize;

/// Shared HTTP client and API credentials container.
///
/// Encapsulates all external API interactions with connection pooling
/// and timeout management.
pub struct ApiService {
    client: Client,
    exchange_token: Option<String>,
}

impl ApiService {
    /// Creates a new `ApiService` with an optional exchangerate API token.
    ///
    /// # Panics
    ///
    /// Panics if the HTTP client cannot be constructed (rare - indicates
    /// invalid TLS configuration or system resource exhaustion).
    pub fn new(exchange_token: Option<String>) -> Self {
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .expect("Failed to build HTTP client"),
            exchange_token,
        }
    }

    /// Generic HTTP GET with JSON deserialization.
    ///
    /// Centralizes request/response error handling with structured logging.
    async fn fetch_json<T>(&self, url: &str) -> Result<T, ServiceError>
    where
        T: for<'de> Deserialize<'de>,
    {
        self.client
            .get(url)
            .send()
            .await
            .map_err(|e| ServiceError::network(url, e))?
            .json::<T>()
            .await
            .map_err(|e| ServiceError::parse(url, e))
    }

    /// Retrieves current weather conditions for a given city.
    ///
    /// # Errors
    ///
    /// Returns `ServiceError::NotFound` if the city doesn't exist,
    /// or propagates network/parse errors from `fetch_json`.
    pub async fn get_weather(&self, city: &str) -> Result<WeatherData, ServiceError> {
        let url = format!(
            "https://wttr.in/{}?format=j1",
            urlencoding::encode(city)
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| ServiceError::network(&url, e))?;

        match response.status() {
            StatusCode::NOT_FOUND => Err(ServiceError::not_found("City", city)),
            StatusCode::OK => response
                .json::<WeatherData>()
                .await
                .map_err(|e| ServiceError::parse(&url, e)),
            status => Err(ServiceError::unexpected_status(&url, status)),
        }
    }

    /// Fetches a random joke from JokeAPI.
    ///
    /// # Errors
    ///
    /// Propagates network or parsing errors as `ServiceError`.
    pub async fn get_joke(&self) -> Result<String, ServiceError> {
        const JOKE_URL: &str = "https://v2.jokeapi.dev/joke/Any?safe-mode&type=single,twopart";

        let joke = self.fetch_json::<JokeResponse>(JOKE_URL).await?;

        Ok(match joke {
            JokeResponse::Single { joke } => joke,
            JokeResponse::TwoPart { setup, delivery } => format!("{}\n{}", setup, delivery),
        })
    }

    /// Converts currency using exchangerate.host API.
    ///
    /// # Errors
    ///
    /// Returns `ServiceError::MissingToken` if API key is not configured,
    /// `ServiceError::Api` for upstream errors, or propagates network/parse errors.
    pub async fn convert_currency(
        &self,
        amount: f64,
        from: &str,
        to: &str,
    ) -> Result<f64, ServiceError> {
        let token = self
            .exchange_token
            .as_ref()
            .ok_or_else(|| ServiceError::missing_token("EXCHANGERATE_TOKEN"))?;

        let url = format!(
            "https://api.exchangerate.host/convert?access_key={}&from={}&to={}&amount={}",
            token,
            from.to_uppercase(),
            to.to_uppercase(),
            amount
        );

        let data = self.fetch_json::<ExchangerateResponse>(&url).await?;

        if data.success {
            data.result
                .ok_or_else(|| ServiceError::missing_field("result", &url))
        } else {
            Err(ServiceError::api_error(
                &url,
                data.error.as_ref().map(|e| e.info.as_str()),
            ))
        }
    }
}