use eframe::NativeOptions;
use egui::{ViewportBuilder, vec2};

mod app;
mod components;
mod i18n;
mod state;
mod theme;
mod views;

fn main() -> eframe::Result {
    kiosk_core::logger::init();
    kiosk_api::updater::cleanup_old_binary();

    // TODO: make it fullscreen on production
    let options = NativeOptions {
        viewport: ViewportBuilder {
            inner_size: Some(vec2(1080.0, 1000.0)),
            resizable: Some(false),
            // fullscreen: Some(true),
            ..Default::default()
        },
        ..Default::default()
    };

    eframe::run_native(
        "Kiosk",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<state::State>::default())
        }),
    )
}
