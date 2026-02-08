use kiosk_api::stations::Station;

#[derive(Default, PartialEq)]
pub enum TripKind {
    #[default]
    OneWay,
    Round,
}

pub struct State {
    pub kind: TripKind,
    pub source: Option<Station>,
    pub destination: Option<Station>,
    pub selected: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            kind: TripKind::OneWay,
            source: None,
            destination: None,
            selected: false,
        }
    }
}
