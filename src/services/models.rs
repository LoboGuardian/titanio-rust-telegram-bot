// src/services/models.rs

//! Data transfer objects for third-party API responses.

use serde::Deserialize;

/// Top-level weather data from wttr.in API.
#[derive(Debug, Deserialize, Clone)]
pub struct WeatherData {
    pub current_condition: Vec<CurrentCondition>,
}

/// Current weather snapshot.
#[derive(Debug, Deserialize, Clone)]
pub struct CurrentCondition {
    #[serde(rename = "temp_C")]
    pub temp_c: String,

    #[serde(rename = "weatherDesc")]
    pub weather_desc: Vec<WeatherDesc>,
}

/// Weather description wrapper.
#[derive(Debug, Deserialize, Clone)]
pub struct WeatherDesc {
    pub value: String,
}

/// JokeAPI response variants.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum JokeResponse {
    Single { joke: String },
    TwoPart { setup: String, delivery: String },
}

/// exchangerate.host API response.
#[derive(Debug, Deserialize)]
pub struct ExchangerateResponse {
    pub success: bool,
    pub result: Option<f64>,
    pub error: Option<ErrorData>,
}

/// API error details from exchangerate.host.
#[derive(Debug, Deserialize)]
pub struct ErrorData {
    /// Numeric error code from the API.
    #[allow(dead_code)]
    pub code: i32,
    
    /// Human-readable error description.
    pub info: String,
}