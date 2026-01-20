mod app;
mod logger;
mod updater;
mod utils;

fn main() -> eframe::Result {
    logger::init();

    // Clean up old binary from previous update
    updater::cleanup_old_binary();

    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Kiosk",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<app::State>::default())
        }),
    )
}
