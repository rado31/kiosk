use eframe::egui;
use egui::{Frame, Margin};

use crate::app::routes::Route;

mod components;
mod constants;
mod routes;
mod views;

#[derive(Default, Clone, Copy, PartialEq)]
pub enum Language {
    #[default]
    Turkmen,
    Russian,
}

#[derive(Default)]
pub struct State {
    pub current_route: Route,
    pub language: Language,
}

impl eframe::App for State {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
