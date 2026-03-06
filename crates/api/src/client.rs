use std::{sync::OnceLock, time::Duration};

use core::config;
use ureq::Agent;

static AGENT: OnceLock<Agent> = OnceLock::new();

fn agent() -> &'static Agent {
    AGENT.get_or_init(|| {
        Agent::config_builder()
            .timeout_global(Some(Duration::from_secs(5)))
            .build()
            .into()
    })
}

pub fn get(path: &str) -> Result<ureq::Body, ureq::Error> {
    let url = format!("{}{}", config::API_BASE_URL, path);

    agent()
        .get(&url)
        .header("X-Device-Id", config::DEVICE_ID)
        .header("Bearer", config::API_KEY)
        .header("ClientID", config::CLIENT_ID)
        .call()
        .map(|res| res.into_body())
}
