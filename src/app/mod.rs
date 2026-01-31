use eframe::{App, Frame};
use egui::Context;

mod components;
mod constants;
mod i18n;
mod pages;
mod prepare;
mod services;
mod state;
mod views;

pub use services::updater;
pub use state::State;

impl App for State {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.config_style(ctx);
        self.check_receiver_of_update();
        self.fetch_stations(ctx);
        self.render_ui(ctx);
    }
}
