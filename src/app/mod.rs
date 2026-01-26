use eframe::egui;
use egui::{CentralPanel, Frame};

use crate::error;

mod components;
mod constants;
mod i18n;
mod pages;
mod routes;
mod services;
mod state;

pub use services::updater;
use services::updater::{UpdateMessage, UpdateStatus};
pub use state::State;

impl eframe::App for State {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Check for update messages from background thread
        if let Some(receiver) = &self.update_receiver {
            while let Ok(msg) = receiver.try_recv() {
                match msg {
                    UpdateMessage::Progress(progress) => {
                        self.update_status = UpdateStatus::Downloading(progress);
                    }
                    UpdateMessage::Downloaded(path) => {
                        self.update_receiver = None;
                        self.update_status = UpdateStatus::Idle;

                        if let Err(e) = services::updater::install_and_restart(&path) {
                            error!("{e}");
                        }

                        break;
                    }
                    UpdateMessage::Done => {
                        self.update_receiver = None;
                        self.update_status = UpdateStatus::Idle;
                        break;
                    }
                }
            }
        }

        let mut style = (*ctx.style()).clone();

        style.spacing.item_spacing.x = 0.0;
        style.interaction.selectable_labels = false;

        ctx.set_style(style);

        components::header::show(ctx, self);

        let container = Frame::new().fill(constants::colors::BG).inner_margin(30.0);

        CentralPanel::default()
            .frame(container)
            .show(ctx, |ui| routes::router(self, ctx, ui));

        if let UpdateStatus::Downloading(ref progress) = self.update_status {
            components::updater_modal::show(ctx, progress);
        }
    }
}
