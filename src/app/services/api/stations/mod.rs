use std::{
    sync::mpsc::{Receiver, channel},
    thread,
};

use crate::error;

use super::{ApiRes, Fetcher};

pub mod types;

pub fn get_all() -> Receiver<Option<Vec<types::Station>>> {
    let (tx, rx) = channel();

    thread::spawn(move || {
        let url = "";
        let data = Fetcher::new()
            .get(url)
            .inspect_err(|e| {
                error!("Get stations. {e}");
                tx.send(None).ok();
            })
            .ok()?;

        let parsed_value: ApiRes<types::Data> = data
            .json()
            .inspect_err(|e| {
                error!("Parse StationRes. {e}");
                tx.send(None).ok();
            })
            .ok()?;

        if let Some(data) = parsed_value.data {
            tx.send(Some(data.stations)).ok();
        }

        if let Some(_e) = parsed_value.error {
            tx.send(None).ok();
        }

        Some(())
    });

    rx
}
