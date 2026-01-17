use super::*;

pub fn show(frame: Frame, ui: &mut Ui, width: f32) {
    frame.show(ui, |ui| {
        ui.set_width(width);
        ui.vertical_centered(|ui| ui.image(include_image!("../../../assets/call_center.svg")))
    });
}
