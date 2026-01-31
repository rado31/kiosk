use std::sync::mpsc::Receiver;

use crate::{app::services::api::stations::types::Station, errors::Result};

#[derive(Default)]
pub struct State {
    pub is_fetching: bool,
    pub has_fetched: bool,
    pub data: Option<Vec<Station>>,
    pub receiver: Option<Receiver<Option<Vec<Station>>>>,
}
