use std::sync::mpsc;

use super::{UpdateMessage, UpdateStatus, routes::Route};

#[derive(Default, Clone, Copy, PartialEq)]
pub enum Language {
    #[default]
    Turkmen,
    Russian,
}

#[derive(Default)]
pub struct State {
    pub route: Route,
    pub language: Language,
    pub update_status: UpdateStatus,
    pub update_receiver: Option<mpsc::Receiver<UpdateMessage>>,
}
