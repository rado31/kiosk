use super::*;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn show(frame: Frame, ui: &mut Ui, width: f32) {
    frame.show(ui, |ui| {
        ui.set_width(width);
        ui.horizontal(|ui| {
            ui.image(include_image!("../../../assets/logo.svg"));
            ui.add_space(10.0);
            ui.label(
                RichText::new(format!("{}", VERSION))
                    .size(14.0)
                    .color(colors::FG_MUTED),
            );
        });
    });
}
