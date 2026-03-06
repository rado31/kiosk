use core::Result;

use crate::{client, response::ApiResponse};

mod types;

pub use types::*;

pub fn fetch<'a>(params: Params<'a>) -> Result<Vec<Trip>> {
    let path = format!(
        "/trips?source={}&destination={}&date={}&adult={}&child={}&client=terminal",
        params.source, params.destination, params.date, params.adult, params.child,
    );

    let mut body = client::get(&path)?;
    let res: ApiResponse<Data> = body.read_json()?;

    if let Some(data) = res.data
        && res.success
    {
        return Ok(data.trips);
    }

    if let Some(e) = res.error {
        return Err(core::AppError::custom(format!(
            "Error on `GET /trips`. {e}"
        )));
    }

    Err(core::AppError::custom("Unknown API error on `GET /trips`"))
}

pub fn fetch_details(params: DetailsParams) -> Result<Vec<TrainWagon>> {
    let path = format!(
        "/cashiers/trips/{}?adult={}&child={}&outbound_wagon_type_id={}",
        params.trip_id, params.adult, params.child, params.wagon_type_id,
    );

    let mut body = client::get(&path)?;
    let res: ApiResponse<Details> = body.read_json()?;

    if let Some(data) = res.data
        && res.success
    {
        return data
            .outbound
            .journeys
            .into_iter()
            .next()
            .map(|j| j.train_wagons)
            .ok_or_else(|| core::AppError::custom("Empty journeys in trip details"));
    }

    if let Some(e) = res.error {
        return Err(core::AppError::custom(format!(
            "Error on `GET /trip_details`. {e}"
        )));
    }

    Err(core::AppError::custom(
        "Unknown API error on `GET /trip_details`",
    ))
}
