use std::sync::mpsc::Receiver;

use kiosk_api::stations::Station;

pub struct State {
    is_fetching: bool,
    has_fetched: bool,
    data: Option<Vec<Station>>,
    receiver: Option<Receiver<Option<Vec<Station>>>>,
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

    pub fn should_fetch(&self) -> bool {
        !self.has_fetched && !self.is_fetching
    }

    pub fn start_fetching(&mut self, receiver: Receiver<Option<Vec<Station>>>) {
        self.is_fetching = true;
        self.receiver = Some(receiver);
    }

    pub fn take_receiver(&mut self) -> Option<Receiver<Option<Vec<Station>>>> {
        self.receiver.take()
    }

    pub fn set_result(&mut self, data: Option<Vec<Station>>) {
        self.data = data;
        self.is_fetching = false;
        self.has_fetched = true;
        self.receiver = None;
    }
}
