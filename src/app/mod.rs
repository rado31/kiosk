use eframe::egui;

use crate::app::routes::Route;

mod components;
mod routes;
mod views;

#[derive(Default)]
pub struct State {
    pub current_route: Route,
    // pub lang: String,
}

impl eframe::App for State {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut style = (*ctx.style()).clone();

        style.spacing.item_spacing.x = 0.0;
        style.interaction.selectable_labels = false;

        ctx.set_style(style);

        components::header::show(ctx);

        let container = egui::Frame::NONE
            .fill(egui::Color32::from_rgb(246, 246, 246))
            .inner_margin(30.0);

        egui::CentralPanel::default()
            .frame(container)
            .show(ctx, |ui| routes::router(self, ui));
    }
}
