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
        let data = Fetcher::new("https://localhost")
            .get("/stations")
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

        if let Some(data) = parsed_value.data
            && parsed_value.success
        {
            tx.send(Some(data.stations)).ok();
        }

        if let Some(e) = parsed_value.error
            && !parsed_value.success
        {
            error!("API error of `GET /stations`. {e}");
            tx.send(None).ok();
        }

        Some(())
    });

    rx
}
