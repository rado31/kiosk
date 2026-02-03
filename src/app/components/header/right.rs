use egui::{Button, Image, Pos2, Stroke, Ui, include_image, vec2};

use crate::app::constants::colors;

use super::Header;

impl<'a> Header<'a> {
    pub fn render_right(&mut self, ui: &mut Ui) {
        let lang_btn_size = vec2(35.0, 35.0);
        let refresh_btn_size = vec2(40.0, 40.0);

        let tm_img = Image::new(include_image!("../../../assets/tm.svg")).fit_to_original_size(1.0);
        let ru_img = Image::new(include_image!("../../../assets/ru.svg")).fit_to_original_size(1.0);
        let refresh_img = Image::new(include_image!("../../../assets/refresh.svg"))
            .fit_to_original_size(1.0)
            .tint(colors::PRIMARY);

        let tm_btn = Button::new(tm_img).min_size(lang_btn_size).frame(false);
        let ru_btn = Button::new(ru_img).min_size(lang_btn_size).frame(false);
        let refresh_btn = Button::new(refresh_img)
            .min_size(refresh_btn_size)
            .fill(colors::BTN_BG_LIGHT)
            .corner_radius(12);

        if ui.add(refresh_btn).clicked() {
            self.state.reset();
        }

        ui.add_space(50.0);

        let ru_response = ui.add(ru_btn);
        let tm_response = ui.add(tm_btn);

        if tm_response.clicked() && !self.state.lang.is_turkmen() {
            self.state.lang.toggle();
        }

        if ru_response.clicked() && self.state.lang.is_turkmen() {
            self.state.lang.toggle();
        }

        let draw_underline = |ui: &mut Ui, points: [Pos2; 2]| {
            ui.painter()
                .line_segment(points, Stroke::new(2.0, colors::PRIMARY));
        };

        if self.state.lang.is_turkmen() {
            draw_underline(
                ui,
                [
                    tm_response.rect.left_bottom(),
                    tm_response.rect.right_bottom(),
                ],
            );
        }

        ui.add_space(10.0);

        if !self.state.lang.is_turkmen() {
            draw_underline(
                ui,
                [
                    ru_response.rect.left_bottom(),
                    ru_response.rect.right_bottom(),
                ],
            );
        }
    }
}
