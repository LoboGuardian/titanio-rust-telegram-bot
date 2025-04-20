// src/services/api.rs
//! A shared API service module to handle all external HTTP-based data fetching for the bot.
//!
//! This includes endpoints for weather, jokes, and currency conversion.
//! Centralizing the API logic here allows command modules to remain clean, modular, and testable.

use log::{error, info};
use reqwest::{Client, StatusCode};
use serde::Deserialize;

/// Top-level weather data returned from the wttr.in API.
#[derive(Debug, Deserialize)]
pub struct WeatherData {
    /// Current weather conditions.
    pub current_condition: Vec<CurrentCondition>,
}

/// A single snapshot of current weather.
#[derive(Debug, Deserialize)]
pub struct CurrentCondition {
    /// Temperature in Celsius.
    #[serde(rename = "temp_C")]
    pub temp_c: String,

    /// Textual description of the weather.
    #[serde(rename = "weatherDesc")]
    pub weather_desc: Vec<WeatherDesc>,
}

/// Weather description, e.g., "Sunny", "Light rain".
#[derive(Debug, Deserialize)]
pub struct WeatherDesc {
    /// Description text.
    pub value: String,
}

/// Response type returned by JokeAPI, which can be either single or two-part.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum JokeResponse {
    /// A single-line joke.
    Single { joke: String },
    /// A setup + delivery style joke.
    TwoPart { setup: String, delivery: String },
}

/// JSON response from exchangerate.host API.
#[derive(Debug, Deserialize)]
pub struct ExchangerateResponse {
    /// Whether the request was successful.
    pub success: bool,
    /// The numeric result of the currency conversion.
    pub result: Option<f64>,
    /// Optional error details if the request failed.
    pub error: Option<ErrorData>,
}

/// Represents an error returned by exchangerate.host.
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct ErrorData {
    /// Numeric error code.
    pub code: i32,
    /// Human-readable explanation of the error.
    pub info: String,
}

/// Shared HTTP client and API credentials container.
pub struct ApiService {
    client: Client,
    /// Shared HTTP client and API credentials container.
    pub exchange_token: Option<String>,
}

impl ApiService {
    /// Creates a new `ApiService` with an optional exchangerate API token.
    pub fn new(exchange_token: Option<String>) -> Self {
        Self {
            client: Client::new(),
            exchange_token,
        }
    }

    /// Generic helper for GET requests that parse JSON responses.
    ///
    /// Logs errors if request or deserialization fails.
    ///
    /// # Type Parameters
    ///
    /// * `T`: The expected type of the parsed JSON response.
    ///
    /// # Arguments
    ///
    /// * `url` - The full URL to send the request to.
    ///
    /// # Errors
    ///
    /// Returns a string-based error if the request fails or parsing fails.
    async fn get_json<T: for<'de> Deserialize<'de>>(&self, url: &str) -> Result<T, String> {
        self.client
            .get(url)
            .send()
            .await
            .map_err(|e| {
                error!("Request failed: {} | URL: {}", e, url);
                "Request failed".to_owned()
            })?
            .json::<T>()
            .await
            .map_err(|e| {
                error!("Failed to parse response: {} | URL: {}", e, url);
                "Parse failed".to_owned()
            })
    }

    /// Retrieves weather data from wttr.in for the given city.
    ///
    /// # Arguments
    ///
    /// * `city` - Name of the city to query.
    ///
    /// # Errors
    ///
    /// Returns a string error if the request fails or the city is not found.
    pub async fn get_weather(&self, city: &str) -> Result<WeatherData, String> {
        let url = format!("https://wttr.in/{}?format=j1", city);
        let resp = self.client.get(&url).send().await.map_err(|e| {
            error!("Weather request failed: {}", e);
            "Request failed".to_owned()
        })?;

        if resp.status() == StatusCode::NOT_FOUND {
            error!("Weather city not found: {}", city);
            return Err("City not found".to_owned());
        }

        resp.json::<WeatherData>().await.map_err(|e| {
            error!("Weather response parsing failed: {}", e);
            "Failed to parse weather data".to_owned()
        })
    }

    /// Fetches a random joke (single-line or two-part) from JokeAPI.
    ///
    /// # Errors
    ///
    /// Returns an error string if the request fails or the response can't be parsed.
    pub async fn get_joke(&self) -> Result<String, String> {
        let url = "https://v2.jokeapi.dev/joke/Any?safe-mode&type=single,twopart";
        let joke = self.get_json::<JokeResponse>(url).await?;

        Ok(match joke {
            JokeResponse::Single { joke } => joke,
            JokeResponse::TwoPart { setup, delivery } => format!("{}\n{}", setup, delivery),
        })
    }

    /// Converts an amount from one currency to another using exchangerate.host.
    ///
    /// Requires a valid API key set via `EXCHANGERATE_TOKEN` in the environment.
    ///
    /// # Arguments
    ///
    /// * `amount` - The amount to convert.
    /// * `from` - The source currency code (e.g., "USD").
    /// * `to` - The target currency code (e.g., "EUR").
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The API token is missing
    /// - The request or parsing fails
    /// - The API itself returns an error
    pub async fn convert_currency(&self, amount: f64, from: &str, to: &str) -> Result<f64, String> {
        let token = match self.exchange_token.as_ref() {
            Some(t) => t,
            None => {
                error!(
                    "Missing EXCHANGERATE_TOKEN | Tried converting: {} {} -> {}",
                    amount, from, to
                );
                return Err("Missing API token".to_owned());
            }
        };

        let url = format!(
            "https://api.exchangerate.host/convert?access_key={}&from={}&to={}&amount={}",
            token, from, to, amount
        );

        let data = self.get_json::<ExchangerateResponse>(&url).await?;

        if data.success {
            Ok(data.result.unwrap_or(0.0))
        } else if let Some(err) = data.error {
            error!(
                "Currency API error: code={}, info='{}' | {} {} -> {}",
                err.code, err.info, amount, from, to
            );
            Err(err.info)
        } else {
            error!("Currency API failed without error message.");
            Err("Unknown error".to_owned())
        }
    }
}
