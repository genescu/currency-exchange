use async_trait::async_trait;
use reqwest::{Client, Error as ReqwestError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::fmt;
use std::path::PathBuf;
use serde_json::{json, Value};

#[derive(Debug, Deserialize, Serialize)]
struct ExchangeRates {
    rates: HashMap<String, f64>,
}

#[async_trait]
pub trait ExchangeRateProvider {
    async fn get_exchange_rate(&self, from: &str, to: &str) -> Result<f64, ExchangeRateError>;
}

#[derive(Debug)]
pub enum ExchangeRateError {
    RequestError(ReqwestError),
    IoError(io::Error),
    RateNotFound(String, String),
    NetworkError(String),
}

impl fmt::Display for ExchangeRateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExchangeRateError::RequestError(err) => write!(f, "Request error: {}", err),
            ExchangeRateError::IoError(err) => write!(f, "IO error: {}", err),
            ExchangeRateError::RateNotFound(from, to) => write!(f, "Exchange rate from {} to {} not found.", from, to),
            ExchangeRateError::NetworkError(err) => write!(f, "Network error: {}", err),
        }
    }
}

impl From<ReqwestError> for ExchangeRateError {
    fn from(error: ReqwestError) -> Self {
        ExchangeRateError::RequestError(error)
    }
}

impl From<io::Error> for ExchangeRateError {
    fn from(error: io::Error) -> Self {
        ExchangeRateError::IoError(error)
    }
}

pub struct ApiExchangeRateProvider {
    pub base_url: String,
    pub client: Client,
}

impl ApiExchangeRateProvider {
    pub fn new(api_url: &str) -> Self {
        ApiExchangeRateProvider {
            base_url: api_url.to_string(),
            client: Client::new(),
        }
    }
}

#[async_trait]
impl ExchangeRateProvider for ApiExchangeRateProvider {
    async fn get_exchange_rate(&self, from: &str, to: &str) -> Result<f64, ExchangeRateError> {
        let url = format!("{}/latest/{}", self.base_url, from);

        // Retry loop for network requests
        let mut attempts = 0;
        let max_attempts = 1;
        while attempts < max_attempts {
            attempts += 1;
            match self.client.get(&url).send().await {
                Ok(response) => {
                    let exchange_rates: ExchangeRates = response.json().await?;
                    save_response_as_json(&exchange_rates)?;
                    return match exchange_rates.rates.get(to) {
                        Some(rate) => Ok(*rate),
                        None => {
                            Err(ExchangeRateError::RateNotFound(from.to_string(), to.to_string()))
                        }
                    };
                },
                Err(err) => {
                    if attempts == max_attempts {
                        return read_from_cache(from, to);
                    }
                }
            }
            //tokio::time::sleep(Duration::from_secs(2)).await;  // Wait before retrying
        }

        Err(ExchangeRateError::NetworkError("Exceeded maximum retries".to_string()))
    }
}

fn save_response_as_json(exchange_rates: &ExchangeRates) -> Result<(), io::Error> {
    let mut file_path = PathBuf::new();
    file_path.push("exchange_rates_backup.json");

    // Serialize exchange_rates to JSON string
    let json_string = serde_json::to_string(exchange_rates)?;

    // Validate JSON by parsing it
    let parsed_json: Value = serde_json::from_str(&json_string)?;

    // Write to file
    let mut file = File::create(file_path)?;
    serde_json::to_writer(&mut file, &parsed_json)?;

    Ok(())
}

fn read_from_cache(from: &str, to: &str) -> Result<f64, ExchangeRateError> {
    let file_path = "exchange_rates_backup.json";
    let mut file = File::open(file_path).map_err(ExchangeRateError::IoError)?;
    let mut data = String::new();
    file.read_to_string(&mut data).map_err(ExchangeRateError::IoError)?;

    // Handle serde_json::Error correctly
    let exchange_rates: ExchangeRates = serde_json::from_str(&data)
        .map_err(|err| ExchangeRateError::IoError(io::Error::new(io::ErrorKind::Other, err)))?;

    match exchange_rates.rates.get(to) {
        Some(rate) => Ok(*rate),
        None => Err(ExchangeRateError::RateNotFound(from.to_string(), to.to_string())),
    }
}
