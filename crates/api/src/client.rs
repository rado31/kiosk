use std::time::Duration;

use core::config;
use ureq::Agent;

pub struct HttpClient {
    agent: Agent,
    base_url: &'static str,
}

impl HttpClient {
    pub fn new() -> Self {
        let agent = Agent::config_builder()
            .timeout_global(Some(Duration::from_secs(30)))
            .build()
            .into();

        Self {
            agent,
            base_url: config::API_BASE_URL,
        }
    }

    pub fn get(&self, path: &str) -> Result<ureq::Body, ureq::Error> {
        let url = format!("{}{}", self.base_url, path);

        self.agent
            .get(&url)
            .header("X-Device-Id", config::DEVICE_ID)
            .header("Bearer", config::API_KEY)
            .header("ClientID", config::CLIENT_ID)
            .call()
            .map(|res| res.into_body())
    }
}
