use serde::Deserialize;

// Trip

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
pub struct Data {
    pub trips: Vec<Trip>,
}

// Trip's Details
#[derive(Deserialize, Debug, Clone)]
pub struct Seat {
    pub id: u32,
    pub label: String,
    pub available: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TrainWagon {
    pub id: u32,
    pub number: u32,
    pub wagon_type_title: String,
    pub wagon_type_id: u32,
    pub seats: Vec<Seat>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DetailsJourney {
    pub train_wagons: Vec<TrainWagon>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Outbound {
    pub trip_id: u32,
    pub journeys: Vec<DetailsJourney>,
}

#[derive(Deserialize, Debug)]
pub struct Details {
    pub outbound: Outbound,
}

// Params
pub struct Params<'a> {
    pub source: u32,
    pub destination: u32,
    pub date: &'a str,
    pub adult: u32,
    pub child: u32,
}

pub struct DetailsParams {
    pub trip_id: u32,
    pub adult: u8,
    pub child: u8,
    pub wagon_type_id: u32,
}
