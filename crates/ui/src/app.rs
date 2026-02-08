use std::{sync::mpsc, thread};

use eframe::{App, Frame};
use egui::{CentralPanel, Context};

use crate::{
    components,
    state::{
        State,
        update::{DownloadProgress, UpdateMessage, UpdateStatus},
    },
    theme::colors,
    views,
};

impl App for State {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.config_style(ctx);
        self.poll_update();
        self.poll_stations(ctx);
        self.render_ui(ctx);
    }
}

impl State {
    pub fn config_style(&self, ctx: &Context) {
        let mut style = (*ctx.style()).clone();

        style.spacing.item_spacing.x = 0.0;
        style.interaction.selectable_labels = false;

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

    pub fn poll_update(&mut self) {
        if let Some(receiver) = &self.update.receiver {
            while let Ok(msg) = receiver.try_recv() {
                match msg {
                    UpdateMessage::Progress(progress) => {
                        self.update.status = UpdateStatus::Downloading(progress);
                    }
                    UpdateMessage::Downloaded(path) => {
                        self.update.receiver = None;
                        self.update.status = UpdateStatus::Idle;

                        if let Err(e) = kiosk_api::updater::install_and_restart(&path) {
                            log::error!("{e}");
                        }

                        break;
                    }
                    UpdateMessage::Done => {
                        self.update.receiver = None;
                        self.update.status = UpdateStatus::Idle;
                        break;
                    }
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn start_update_check(&mut self, ctx: &Context) {
        let (tx, rx) = mpsc::channel();
        let ctx = ctx.clone();

        self.update.receiver = Some(rx);
        self.update.status = UpdateStatus::Checking;

        thread::spawn(move || {
            let opt_info = match kiosk_api::updater::check() {
                Ok(v) => v,
                Err(e) => {
                    log::error!("Check new update. {e}");
                    tx.send(UpdateMessage::Done).ok();
                    return;
                }
            };

            if let Some(info) = opt_info {
                let tx_progress = tx.clone();
                let ctx_progress = ctx.clone();
                let version = info.version.clone();

                let res = kiosk_api::updater::download(&info, |downloaded, total| {
                    tx_progress
                        .send(UpdateMessage::Progress(DownloadProgress {
                            downloaded,
                            total,
                            version: version.clone(),
                        }))
                        .ok();

                    ctx_progress.request_repaint();
                });

                match res {
                    Ok(path) => {
                        tx.send(UpdateMessage::Downloaded(path)).ok();
                    }
                    Err(e) => {
                        log::error!("Download update. {e}");
                        tx.send(UpdateMessage::Done).ok();
                    }
                };

                ctx.request_repaint();
            } else {
                tx.send(UpdateMessage::Done).ok();
                ctx.request_repaint();
            };
        });
    }

    pub fn poll_stations(&mut self, ctx: &Context) {
        if self.stations.should_fetch() {
            let (tx, rx) = mpsc::channel();

            self.stations.start_fetching(rx);

            thread::spawn(move || {
                match kiosk_api::stations::fetch_all() {
                    Ok(stations) => {
                        tx.send(Some(stations)).ok();
                    }
                    Err(e) => {
                        log::error!("Fetch stations. {e}");
                        tx.send(None).ok();
                    }
                }
            });
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
        components::header::show(self, ctx);

        let container = egui::Frame::new().fill(colors::BG).inner_margin(20.0);

        CentralPanel::default().frame(container).show(ctx, |ui| {
            views::view(self, ctx, ui);
        });

        if let Some(progress) = self.update.status.downloading() {
            components::updater_modal::open(ctx, progress);
        }
    }
}
