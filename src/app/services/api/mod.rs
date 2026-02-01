use std::{fmt, time::Duration};

use reqwest::{
    blocking::{Client, ClientBuilder, Response},
    header::{HeaderMap, HeaderValue},
};
use serde::Deserialize;

use crate::errors::Result;

pub mod stations;

pub struct Fetcher<'a> {
    client: Client,
    base_url: &'a str,
}

impl<'a> Fetcher<'a> {
    pub fn new(base_url: &'a str) -> Self {
        let mut headers = HeaderMap::new();
        let val = |value: &'static str| HeaderValue::from_static(value);

        headers.insert("X-Device-Id", val("REDACTED_DEVICE_ID"));
        headers.insert("Bearer", val("sadfkskfjlskflsfk"));
        headers.insert("ClientID", val("REDACTED_CLIENT_ID"));

        let client = ClientBuilder::new()
            .default_headers(headers)
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap(); // safe, because I don't use TLS configuration, see the docs

        Self { client, base_url }
    }

    fn join_url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    pub fn get(&self, path: &str) -> Result<Response> {
        Ok(self.client.get(self.join_url(path)).send()?)
    }
}

#[derive(Deserialize, Debug)]
pub struct ApiRes<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<ApiResError>,
}

#[derive(Deserialize, Debug)]
pub struct ApiResError {
    pub id: String,
    pub message: String,
}

impl fmt::Display for ApiResError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ID: {}, Message: {}", self.id, self.message)
    }
}
