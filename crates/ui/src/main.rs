use eframe::NativeOptions;
use egui::{FontData, FontDefinitions, FontFamily, ViewportBuilder, vec2};

mod app;
mod components;
mod i18n;
mod state;
mod theme;
mod views;

fn main() -> eframe::Result {
    core::logger::init();
    api::updater::cleanup_old_binary();

    // TODO: make it fullscreen on production
    let options = NativeOptions {
        viewport: ViewportBuilder {
            // inner_size: Some(vec2(1080.0, 1000.0)),
            resizable: Some(false),
            fullscreen: Some(true),
            ..Default::default()
        },
        ..Default::default()
    };

    eframe::run_native(
        "Kiosk",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            let mut fonts = FontDefinitions::default();

            fonts.font_data.insert(
                "Inter".to_owned(),
                FontData::from_static(include_bytes!("assets/Inter.ttf")).into(),
            );

            fonts.font_data.insert(
                "InterBold".to_owned(),
                FontData::from_static(include_bytes!("assets/InterBold.ttf")).into(),
            );

            fonts
                .families
                .get_mut(&FontFamily::Proportional)
                .unwrap()
                .insert(0, "Inter".to_owned());

            fonts.families.insert(
                FontFamily::Name("bold".into()),
                vec!["InterBold".to_owned()],
            );

            cc.egui_ctx.set_fonts(fonts);

            Ok(Box::<state::State>::default())
        }),
    )
}
