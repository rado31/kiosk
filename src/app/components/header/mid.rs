use egui::{Image, Ui, include_image};

use super::Header;

impl<'a> Header<'a> {
    pub fn render_mid(&self, ui: &mut Ui) {
        let img = Image::new(include_image!("../../../assets/call_center.svg"));
        ui.add(img);
    }
}
