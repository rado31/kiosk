mod app;
mod errors;
mod logger;
mod utils;

fn main() -> eframe::Result {
    logger::init();
    app::updater::cleanup_old_binary();

    std::thread::spawn(|| app::services::api::stations::get_all());

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
