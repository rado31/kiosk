use egui::{
    Align, Button, Context, Frame, Image, Layout, Margin, Pos2, RichText, Shadow, Stroke,
    TopBottomPanel, Ui, include_image, vec2,
};

use crate::{state::State, theme::colors, views::View};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn show(state: &mut State, ctx: &Context) {
    let frame = Frame::new()
        .fill(colors::WHITE)
        .shadow(Shadow {
            offset: [0, 1],
            blur: 4,
            spread: 0,
            color: colors::SHADOW,
        })
        .inner_margin(Margin::symmetric(20, 10));

    let top_panel = TopBottomPanel::top("top_panel")
        .frame(frame)
        .show_separator_line(false);

    top_panel.show(ctx, |ui| {
        ui.columns_const(|[col1, col2, col3]| {
            col1.horizontal_centered(render_left);
            col2.centered_and_justified(render_mid);
            col3.with_layout(Layout::right_to_left(Align::Center), |ui| {
                render_right(state, ui);
            });
        });
    });
}

fn render_left(ui: &mut Ui) {
    ui.add(Image::new(include_image!("../assets/logo.svg")));
    ui.add_space(30.0);

    let version = RichText::new(VERSION).size(14.0).color(colors::FG_MUTED);

    ui.label(version);
}

fn render_mid(ui: &mut Ui) {
    let img = Image::new(include_image!("../assets/call_center.svg"));
    ui.add(img);
}

fn render_right(state: &mut State, ui: &mut Ui) {
    let lang_btn_size = vec2(35.0, 35.0);
    let refresh_btn_size = vec2(40.0, 40.0);

    let tm_img = Image::new(include_image!("../assets/tm.svg")).fit_to_original_size(1.0);
    let ru_img = Image::new(include_image!("../assets/ru.svg")).fit_to_original_size(1.0);
    let refresh_img = Image::new(include_image!("../assets/refresh.svg"))
        .fit_to_original_size(1.0)
        .tint(colors::PRIMARY);

    let tm_btn = Button::new(tm_img).min_size(lang_btn_size).frame(false);
    let ru_btn = Button::new(ru_img).min_size(lang_btn_size).frame(false);
    let refresh_btn = Button::new(refresh_img)
        .min_size(refresh_btn_size)
        .fill(colors::BTN_BG_LIGHT)
        .corner_radius(12);

    if state.current_view() == View::Home && ui.add(refresh_btn).clicked() {
        state.reset();
    }

    ui.add_space(50.0);

    let ru_response = ui.add(ru_btn);
    let tm_response = ui.add(tm_btn);

    if tm_response.clicked() && !state.lang.is_turkmen() {
        state.lang.toggle();
    }

    if ru_response.clicked() && state.lang.is_turkmen() {
        state.lang.toggle();
    }

    let draw_underline = |ui: &mut Ui, points: [Pos2; 2]| {
        ui.painter()
            .line_segment(points, Stroke::new(2.0, colors::PRIMARY));
    };

    if state.lang.is_turkmen() {
        draw_underline(
            ui,
            [
                tm_response.rect.left_bottom(),
                tm_response.rect.right_bottom(),
            ],
        );
    }

    ui.add_space(10.0);

    if !state.lang.is_turkmen() {
        draw_underline(
            ui,
            [
                ru_response.rect.left_bottom(),
                ru_response.rect.right_bottom(),
            ],
        );
    }
}
