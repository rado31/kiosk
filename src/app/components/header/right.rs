use super::*;

pub fn show(frame: Frame, ui: &mut Ui, width: f32, state: &mut State) {
    frame.show(ui, |ui| {
        ui.set_width(width);

        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            let restart_img_size = vec2(20.0, 20.0);
            let restart_btn_size = vec2(40.0, 40.0);
            let lang_img_size = vec2(25.0, 25.0);
            let lang_btn_size = vec2(35.0, 35.0);

            let restart_svg = include_image!("../../../assets/restart.svg");
            let tm_svg = include_image!("../../../assets/tm.svg");
            let ru_svg = include_image!("../../../assets/ru.svg");

            let restart_img = Image::new(restart_svg)
                .fit_to_exact_size(restart_img_size)
                .tint(constants::PRIMARY);
            let tm_img = Image::new(tm_svg).fit_to_exact_size(lang_img_size);
            let ru_img = Image::new(ru_svg).fit_to_exact_size(lang_img_size);

            let restart_btn = Button::new(restart_img)
                .min_size(restart_btn_size)
                .fill(constants::BTN_BG_LIGHT)
                .corner_radius(12);
            let tm_btn = Button::new(tm_img).min_size(lang_btn_size).frame(false);
            let ru_btn = Button::new(ru_img).min_size(lang_btn_size).frame(false);

            if ui.add(restart_btn).clicked() {
                println!("Restarted")
            };

            ui.add_space(50.0);

            let ru_response = ui.add(ru_btn);
            let tm_response = ui.add(tm_btn);

            if ru_response.clicked() {
                state.language = Language::Russian;
            }

            if tm_response.clicked() {
                state.language = Language::Turkmen;
            }

            if state.language == Language::Turkmen {
                ui.painter().line_segment(
                    [
                        tm_response.rect.left_bottom(),
                        tm_response.rect.right_bottom(),
                    ],
                    Stroke::new(2.0, constants::PRIMARY),
                );
            }

            ui.add_space(10.0);

            if state.language == Language::Russian {
                ui.painter().line_segment(
                    [
                        ru_response.rect.left_bottom(),
                        ru_response.rect.right_bottom(),
                    ],
                    Stroke::new(2.0, constants::PRIMARY),
                );
            }
        })
    });
}
