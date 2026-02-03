use egui::{CentralPanel, Context, Frame};

use crate::{
    app::{
        State, components,
        constants::colors,
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

        // spacing
        style.spacing.item_spacing.x = 0.0;
        style.interaction.selectable_labels = false;

        // scroll
        style.spacing.scroll.floating = true;
        style.spacing.scroll.floating_width = 6.0;
        style.spacing.scroll.floating_allocated_width = 0.0;
        style.spacing.scroll.foreground_color = true;
        style.spacing.scroll.active_background_opacity = 0.0;
        style.spacing.scroll.active_handle_opacity = 0.3;
        style.spacing.scroll.interact_background_opacity = 0.05;
        style.spacing.scroll.interact_handle_opacity = 0.7;

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
        if self.stations.should_fetch() {
            self.stations.start_fetching(api::stations::get_all());
        }

        if let Some(rx) = self.stations.take_receiver() {
            let mut received = false;

            while let Ok(data) = rx.try_recv() {
                received = true;
                self.stations.set_result(data);
                ctx.request_repaint();
            }

            if !received {
                self.stations.start_fetching(rx);
            }
        }
    }

    pub fn render_ui(&mut self, ctx: &Context) {
        components::Header::new(self, ctx).show();

        let container = Frame::new().fill(colors::BG).inner_margin(20.0);

        CentralPanel::default().frame(container).show(ctx, |ui| {
            views::view(self, ctx, ui);
        });

        if let Some(progress) = self.new_update.status.downloading() {
            components::updater_modal::open(ctx, progress);
        }
    }
}
