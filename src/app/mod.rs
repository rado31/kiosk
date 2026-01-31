use eframe::{Frame as Eframe, egui};
use egui::{CentralPanel, Frame};

use crate::debug;

mod components;
mod constants;
mod i18n;
mod pages;
pub mod services;
mod state;
mod views;

pub use components::header::Header;
pub use services::updater;
use services::updater::{NewUpdate, UpdateStatus, check_receiver_of_update};
pub use state::State;

impl eframe::App for State {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Eframe) {
        check_receiver_of_update(self);

        if !self.stations.has_fetched && !self.stations.is_fetching {
            self.stations.is_fetching = true;
            self.stations.receiver = Some(services::api::stations::get_all());
        }

        if let Some(rx) = &self.stations.receiver {
            while let Ok(data) = rx.try_recv() {
                match data {
                    Some(stations) => self.stations.data = Some(stations),
                    None => {}
                }

                self.stations.is_fetching = false;
                self.stations.has_fetched = true;

                ctx.request_repaint();
            }
        }

        if self.stations.has_fetched {
            self.stations.receiver = None;
        }

        let mut style = (*ctx.style()).clone();

        style.spacing.item_spacing.x = 0.0;
        style.interaction.selectable_labels = false;

        ctx.set_style(style);

        Header::new(self, ctx).show();

        let container = Frame::new().fill(constants::colors::BG).inner_margin(30.0);

        CentralPanel::default()
            .frame(container)
            .show(ctx, |ui| views::view(self, ctx, ui));

        if let UpdateStatus::Downloading(ref progress) = self.new_update.status {
            components::updater_modal::show(ctx, progress);
        }
    }
}
