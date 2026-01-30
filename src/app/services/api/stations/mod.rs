use serde::Deserialize;

use crate::{debug, error};

use super::Fetcher;

mod types;

pub fn get_all() {
    let url = "";
    let res = Fetcher::new().get(url);

    match res {
        Ok(data) => {
            let stations: types::StationsRes = data.json().unwrap();

            debug!("{:#?}", stations);
        }
        Err(error) => error!("{error}"),
    };
}
