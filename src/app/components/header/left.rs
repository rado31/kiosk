use egui::{Image, RichText, Ui, include_image};

use crate::app::constants::colors;

use super::Header;

const VERSION: &str = env!("CARGO_PKG_VERSION");

impl<'a> Header<'a> {
    pub fn render_left(&self, ui: &mut Ui) {
        ui.add(Image::new(include_image!("../../../assets/logo.svg")));
        ui.add_space(30.0);

        let version = RichText::new(format!("{}", VERSION))
            .size(14.0)
            .color(colors::FG_MUTED);

        ui.label(version);
    }
}
