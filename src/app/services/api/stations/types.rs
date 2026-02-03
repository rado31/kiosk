use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Station {
    pub id: u32,
    pub title_tm: String,
    pub title_ru: String,
}

impl Station {
    pub fn placeholder_source() -> Self {
        Self {
            id: 0,
            title_tm: "Nireden".to_owned(),
            title_ru: "Откуда".to_owned(),
        }
    }

    pub fn placeholder_destination() -> Self {
        Self {
            id: 0,
            title_tm: "Nirä".to_owned(),
            title_ru: "Куда".to_owned(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Data {
    pub stations: Vec<Station>,
}
