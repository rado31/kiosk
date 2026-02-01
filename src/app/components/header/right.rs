use egui::{Button, Image, Pos2, Stroke, Ui, include_image, vec2};

use crate::app::{State, constants::colors};

use super::Header;

impl<'a> Header<'a> {
    pub fn render_right(&mut self, ui: &mut Ui) {
        let lang_btn_size = vec2(35.0, 35.0);
        let restart_btn_size = vec2(40.0, 40.0);

        let tm_img = Image::new(include_image!("../../../assets/tm.svg")).fit_to_original_size(1.0);
        let ru_img = Image::new(include_image!("../../../assets/ru.svg")).fit_to_original_size(1.0);
        let restart_img = Image::new(include_image!("../../../assets/restart.svg"))
            .fit_to_original_size(1.0)
            .tint(colors::PRIMARY);

        let tm_btn = Button::new(tm_img).min_size(lang_btn_size).frame(false);
        let ru_btn = Button::new(ru_img).min_size(lang_btn_size).frame(false);
        let restart_btn = Button::new(restart_img)
            .min_size(restart_btn_size)
            .fill(colors::BTN_BG_LIGHT)
            .corner_radius(12);

        if ui.add(restart_btn).clicked() {
            *self.state = State::default();
        }

        ui.add_space(50.0);

        let ru_response = ui.add(ru_btn);
        let tm_response = ui.add(tm_btn);

        if tm_response.clicked() && !self.state.is_turkmen_lang() {
            self.state.toggle_lang();
        }

        if ru_response.clicked() && self.state.is_turkmen_lang() {
            self.state.toggle_lang();
        }

        let draw_underline = |ui: &mut Ui, points: [Pos2; 2]| {
            ui.painter()
                .line_segment(points, Stroke::new(2.0, colors::PRIMARY));
        };

        if self.state.is_turkmen_lang() {
            draw_underline(
                ui,
                [
                    tm_response.rect.left_bottom(),
                    tm_response.rect.right_bottom(),
                ],
            );
        }

        ui.add_space(10.0);

        if !self.state.is_turkmen_lang() {
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
