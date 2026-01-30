use super::*;

#[derive(Deserialize, Debug)]
pub struct Station {
    pub id: u32,
    pub title_tm: String,
    pub title_ru: String,
}

#[derive(Deserialize, Debug)]
pub struct Stations {
    pub stations: Vec<Station>,
}

#[derive(Deserialize, Debug)]
pub struct StationsRes {
    pub success: bool,
    pub data: Stations,
}
