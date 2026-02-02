use std::sync::mpsc::Receiver;

use crate::app::services::api::stations::types::Station;

pub struct State {
    pub is_fetching: bool,
    pub has_fetched: bool,
    pub data: Option<Vec<Station>>,
    pub receiver: Option<Receiver<Option<Vec<Station>>>>,
    pub selected_letter: &'static str,
}

impl Default for State {
    fn default() -> Self {
        Self {
            is_fetching: false,
            has_fetched: false,
            data: None,
            receiver: None,
            selected_letter: "A",
        }
    }
}

impl State {
    pub fn get(&self) -> Option<&Vec<Station>> {
        self.data.as_ref()
    }

    pub fn get_letter(&self) -> &str {
        self.selected_letter
    }

    pub fn select_letter(&mut self, letter: &'static str) {
        self.selected_letter = letter;
    }
}
