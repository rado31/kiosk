use core::Result;
use serde::Deserialize;

use crate::{client::HttpClient, response::ApiResponse};

#[derive(Deserialize, Debug, Clone)]
pub struct WagonType {
    pub has_seats: bool,
    pub wagon_type_title: String,
    pub wagon_type_id: u32,
    pub price: f32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Journey {
    pub id: u32,
    pub departure_time: String,
    pub arrival_time: String,
    pub travel_time: u32,
    pub train_run_number: String,
    pub service_type_id: u32,
    pub service_type_title: String,
    pub distance: u32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Trip {
    pub id: u32,
    pub departure_time: String,
    pub arrival_time: String,
    pub travel_time: u32,
    pub distance: u32,
    pub wagon_types: Vec<WagonType>,
    pub journeys: Vec<Journey>,
}

#[derive(Deserialize, Debug)]
struct TripsData {
    trips: Vec<Trip>,
}

pub struct TripsParams<'a> {
    pub source: u32,
    pub destination: u32,
    pub date: &'a str,
    pub adult: u32,
    pub child: u32,
}

pub fn fetch<'a>(params: TripsParams<'a>) -> Result<Vec<Trip>> {
    let path = format!(
        "/trips?source={}&destination={}&date={}&adult={}&child={}&client=terminal",
        params.source, params.destination, params.date, params.adult, params.child,
    );

    let client = HttpClient::new();
    let mut body = client.get(&path)?;
    let res: ApiResponse<TripsData> = body.read_json()?;

    if let Some(data) = res.data
        && res.success
    {
        return Ok(data.trips);
    }

    if let Some(e) = res.error {
        return Err(core::AppError::custom(format!("API error. {e}")));
    }

    Err(core::AppError::custom("Unknown API error"))
}
