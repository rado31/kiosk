use super::*;

pub fn show(ui: &mut Ui, state: &mut State) {
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

    let is_updating = !matches!(state.new_update.status, UpdateStatus::Idle);

    if ui.add(restart_btn).clicked() && !is_updating {
        state.new_update.status = UpdateStatus::Checking;
        state.new_update.receiver = Some(updater::start_check(ui.ctx()));
    }

    ui.add_space(50.0);

    let ru_response = ui.add(ru_btn);
    let tm_response = ui.add(tm_btn);

    if tm_response.clicked() && !state.is_turkmen_lang() {
        state.toggle_lang();
    }

    if ru_response.clicked() && state.is_turkmen_lang() {
        state.toggle_lang();
    }

    let show_underline = |ui: &mut Ui, points: [Pos2; 2]| {
        ui.painter()
            .line_segment(points, Stroke::new(2.0, colors::PRIMARY));
    };

    if state.is_turkmen_lang() {
        show_underline(
            ui,
            [
                tm_response.rect.left_bottom(),
                tm_response.rect.right_bottom(),
            ],
        );
    }

    ui.add_space(10.0);

    if !state.is_turkmen_lang() {
        show_underline(
            ui,
            [
                ru_response.rect.left_bottom(),
                ru_response.rect.right_bottom(),
            ],
        );
    }
}
