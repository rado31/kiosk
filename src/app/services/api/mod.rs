use reqwest::{
    Error,
    blocking::{Client, Response},
};

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

    pub fn get(&self, url: &str) -> Result<Response, Error> {
        self.client.get(url).send()
    }
}
