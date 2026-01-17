use super::*;

pub fn show(frame: Frame, ui: &mut Ui, width: f32) {
    frame.show(ui, |ui| {
        ui.set_width(width);
        ui.image(include_image!("../../../assets/logo.svg"));
        ui.add_space(30.0);
        ui.label(RichText::new("2.0.12").size(18.0));
    });
}
