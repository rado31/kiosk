use core::Result;
use serde::Deserialize;

use crate::{client::HttpClient, response::ApiResponse};

#[derive(Deserialize, Debug, Clone)]
pub struct Station {
    pub id: u32,
    pub title_tm: String,
    pub title_ru: String,
}

impl Station {
    pub fn get_title(&self, is_turkmen: bool) -> &str {
        if is_turkmen {
            return &self.title_tm;
        }

        &self.title_ru
    }
}

#[derive(Deserialize, Debug)]
struct StationsData {
    stations: Vec<Station>,
}

/// Fetch all stations from the API (blocking).
pub fn fetch_all() -> Result<Vec<Station>> {
    let client = HttpClient::new();
    let mut body = client.get("/stations")?;
    let res: ApiResponse<StationsData> = body.read_json()?;

    if let Some(data) = res.data
        && res.success
    {
        return Ok(data.stations);
    }

    if let Some(e) = res.error {
        return Err(core::AppError::custom(format!(
            "API error of `GET /stations`. {e}"
        )));
    }

    Err(core::AppError::custom("Unknown API error"))
}
