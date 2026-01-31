use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Station {
    pub id: u32,
    pub title_tm: String,
    pub title_ru: String,
}

#[derive(Deserialize, Debug)]
pub struct Data {
    pub stations: Vec<Station>,
}
