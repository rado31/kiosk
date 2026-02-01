use egui::{CentralPanel, Context, Frame};

use crate::{
    app::{
        State, components, constants,
        services::{
            api,
            updater::{UpdateMessage, UpdateStatus, install_and_restart},
        },
        views,
    },
    error,
};

impl State {
    pub fn config_style(&self, ctx: &Context) {
        let mut style = (*ctx.style()).clone();

        style.spacing.item_spacing.x = 0.0;
        style.interaction.selectable_labels = false;

        ctx.set_style(style);
    }

    pub fn check_receiver_of_update(&mut self) {
        if let Some(receiver) = &self.new_update.receiver {
            while let Ok(msg) = receiver.try_recv() {
                match msg {
                    UpdateMessage::Progress(progress) => {
                        self.new_update.status = UpdateStatus::Downloading(progress);
                    }
                    UpdateMessage::Downloaded(path) => {
                        self.new_update.receiver = None;
                        self.new_update.status = UpdateStatus::Idle;

                        if let Err(e) = install_and_restart(&path) {
                            error!("{e}");
                        }

                        break;
                    }
                    UpdateMessage::Done => {
                        self.new_update.receiver = None;
                        self.new_update.status = UpdateStatus::Idle;
                        break;
                    }
                }
            }
        }
    }

    pub fn fetch_stations(&mut self, ctx: &Context) {
        if !self.stations.has_fetched && !self.stations.is_fetching {
            self.stations.is_fetching = true;
            self.stations.receiver = Some(api::stations::get_all());
        }

        if let Some(rx) = &self.stations.receiver {
            while let Ok(data) = rx.try_recv() {
                match data {
                    Some(stations) => self.stations.data = Some(stations),
                    // TODO: show toast widget with error message
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
    }

    pub fn render_ui(&mut self, ctx: &Context) {
        components::Header::new(self, ctx).show();

        let container = Frame::new().fill(constants::colors::BG).inner_margin(30.0);

        CentralPanel::default()
            .frame(container)
            .show(ctx, |ui| views::view(self, ctx, ui));

        if let Some(progress) = self.new_update.status.downloading() {
            components::updater_modal::open(ctx, progress);
        }
    }
}
