use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Station {
    pub id: u32,
    pub title_tm: String,
    pub title_ru: String,
}

impl Station {
    pub fn new(title_tm: String, title_ru: String) -> Self {
        Self {
            id: 0,
            title_tm,
            title_ru,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Data {
    pub stations: Vec<Station>,
}
