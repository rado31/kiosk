use eframe::egui;
use egui::{CentralPanel, Frame};

mod components;
mod constants;
mod i18n;
mod pages;
mod services;
mod state;
mod views;

pub use services::updater;
use services::updater::{NewUpdate, UpdateMessage, UpdateStatus, check_receiver_of_update};
pub use state::State;

impl eframe::App for State {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        check_receiver_of_update(self);

        let mut style = (*ctx.style()).clone();

        style.spacing.item_spacing.x = 0.0;
        style.interaction.selectable_labels = false;

        ctx.set_style(style);

        components::header::show(ctx, self);

        let container = Frame::new().fill(constants::colors::BG).inner_margin(30.0);

        CentralPanel::default()
            .frame(container)
            .show(ctx, |ui| views::view(self, ctx, ui));

        if let UpdateStatus::Downloading(ref progress) = self.new_update.status {
            components::updater_modal::show(ctx, progress);
        }
    }
}
