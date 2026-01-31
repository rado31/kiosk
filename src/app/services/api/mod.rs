use reqwest::blocking::{Client, Response};
use serde::Deserialize;

use crate::errors::Result;

pub mod stations;

pub struct Fetcher {
    client: Client,
}

impl Fetcher {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub fn get(&self, url: &str) -> Result<Response> {
        Ok(self.client.get(url).send()?)
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
