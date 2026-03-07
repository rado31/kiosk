use std::sync::mpsc::{Receiver, TryRecvError};

use api::stations::Station;

pub struct State {
    is_fetching: bool,
    has_fetched: bool,
    has_error: bool,
    data: Option<Vec<Station>>,
    receiver: Option<Receiver<Option<Vec<Station>>>>,
    pub selected_letter: &'static str,
}

impl Default for State {
    fn default() -> Self {
        Self {
            is_fetching: false,
            has_fetched: false,
            has_error: false,
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

    pub fn retry(&mut self) {
        if !self.has_error {
            return;
        }

        self.has_fetched = false;
        self.has_error = false;
        self.receiver = None;
    }

    pub fn start_fetching(&mut self, receiver: Receiver<Option<Vec<Station>>>) {
        self.is_fetching = true;
        self.receiver = Some(receiver);
    }

    pub fn poll(&mut self) {
        let Some(rx) = &self.receiver else {
            return;
        };

        match rx.try_recv() {
            Ok(data) => self.set_result(data),
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => self.set_result(None),
        };
    }

    pub fn has_error(&self) -> bool {
        self.has_error
    }

    pub fn set_result(&mut self, data: Option<Vec<Station>>) {
        self.has_error = data.is_none();
        self.data = data;
        self.is_fetching = false;
        self.has_fetched = true;
        self.receiver = None;
    }
}
