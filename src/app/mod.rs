use std::{path::PathBuf, sync::mpsc};

use eframe::egui;
use egui::{Frame, Margin};

use crate::{app::routes::Route, updater};

mod components;
mod constants;
mod i18n;
mod routes;
mod views;

#[derive(Default, Clone, Copy, PartialEq)]
pub enum Language {
    #[default]
    Turkmen,
    Russian,
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum UpdateStatus {
    #[default]
    Idle,
    Checking,
    Downloading,
}

#[derive(Default)]
pub struct State {
    pub current_route: Route,
    pub language: Language,
    pub update_status: UpdateStatus,
    pub update_receiver: Option<mpsc::Receiver<Option<PathBuf>>>,
}

impl eframe::App for State {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Check for update result from background thread
        if let Some(receiver) = &self.update_receiver {
            if let Ok(result) = receiver.try_recv() {
                self.update_receiver = None;
                self.update_status = UpdateStatus::Idle;

                if let Some(path) = result {
                    updater::install_and_restart(&path).ok();
                }
            }
        }

        let mut style = (*ctx.style()).clone();

        style.spacing.item_spacing.x = 0.0;
        style.interaction.selectable_labels = false;

        ctx.set_style(style);

        components::header::show(ctx, self);

        let container = Frame::NONE.fill(constants::BG).inner_margin(Margin {
            top: 30 + components::header::HEIGHT,
            left: 30,
            right: 30,
            bottom: 30,
        });

        egui::CentralPanel::default()
            .frame(container)
            .show(ctx, |ui| routes::router(self, ui));
    }
}
