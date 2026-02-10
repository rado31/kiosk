use std::time::Instant;

use api::stations::Station;

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
    searched_at: Option<Instant>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            kind: TripKind::OneWay,
            source: None,
            destination: None,
            selected: false,
            searched_at: None,
        }
    }
}

impl State {
    pub fn search_on_cooldown(&self) -> bool {
        self.searched_at
            .is_some_and(|t| t.elapsed().as_secs_f32() < 5.0)
    }

    pub fn mark_searched(&mut self) {
        self.searched_at = Some(Instant::now());
    }
}
