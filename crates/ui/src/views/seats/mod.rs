use std::sync::mpsc;

use egui::{Align2, FontId, Frame, RichText, Sense, Shadow, Spinner, Stroke, Ui, vec2};

use crate::{
    i18n::t,
    state::State,
    theme::{colors, corners},
    views::View,
};

mod explanation;
mod grid;
mod nav;
mod passengers;

fn poll_details(state: &mut State, ctx: &egui::Context) {
    let Some(rx) = state.seats.take_receiver() else {
        return;
    };

    match rx.try_recv() {
        Ok(result) => {
            state.seats.set_result(result);
            ctx.request_repaint();
        }
        Err(mpsc::TryRecvError::Empty) => {
            state.seats.start_fetching(rx);
            ctx.request_repaint();
        }
        Err(mpsc::TryRecvError::Disconnected) => {
            state.seats.set_result(None);
        }
    }
}

pub fn show(state: &mut State, ctx: &egui::Context, ui: &mut Ui) {
    poll_details(state, ctx);

    let title = RichText::new(t(&state.lang, "choose_seat"))
        .size(30.0)
        .family(egui::FontFamily::Name("bold".into()))
        .color(colors::FG);

    ui.vertical_centered(|ui| ui.label(title));
    ui.add_space(8.0);

    let subtitle = RichText::new(t(&state.lang, "choose_seat_hint"))
        .size(16.0)
        .color(colors::FG_MUTED);

    ui.vertical_centered(|ui| ui.label(subtitle));
    ui.add_space(20.0);

    render_back_button(state, ui);
    ui.add_space(16.0);

    let card = Frame::new()
        .inner_margin(24.0)
        .corner_radius(corners::LARGE)
        .fill(colors::WHITE)
        .shadow(Shadow {
            offset: [0, 2],
            blur: 8,
            spread: 0,
            color: colors::SHADOW,
        });

    card.show(ui, |ui| {
        ui.set_width(ui.available_width());

        explanation::render(state, ui);

        ui.add_space(16.0);
        render_divider(ui);
        ui.add_space(16.0);

        if state.seats.is_fetching() {
            ui.vertical_centered(|ui| {
                ui.add_space(50.0);
                ui.add(Spinner::new().size(50.0).color(colors::PRIMARY));
                ui.add_space(50.0);
            });
        } else if state.seats.has_error {
            let msg = RichText::new(t(&state.lang, "seats_fetch_error"))
                .size(22.0)
                .color(colors::ERROR);
            ui.vertical_centered(|ui| {
                ui.add_space(40.0);
                ui.label(msg);
                ui.add_space(40.0);
            });
        } else if state.seats.get_wagons().is_some() {
            grid::render(state, ui);
            ui.add_space(12.0);
            nav::render(state, ui);
        }

        ui.add_space(16.0);
        render_divider(ui);
        ui.add_space(16.0);

        passengers::render(state, ui);
    });
}

fn render_back_button(state: &mut State, ui: &mut Ui) {
    let shadow = Shadow {
        offset: [0, 2],
        blur: 8,
        spread: 0,
        color: colors::SHADOW,
    };
    let (rect, res) = ui.allocate_exact_size(vec2(160.0, 44.0), Sense::CLICK);

    ui.painter().add(shadow.as_shape(rect, corners::MEDIUM));
    ui.painter()
        .rect_filled(rect, corners::MEDIUM, colors::WHITE);
    ui.painter().text(
        rect.center(),
        Align2::CENTER_CENTER,
        format!("← {}", t(&state.lang, "home")),
        FontId::proportional(16.0),
        colors::FG,
    );

    if res.clicked() {
        state.reset();
        state.go_to(View::Home);
    }
}

fn render_divider(ui: &mut Ui) {
    let (sep, _) = ui.allocate_exact_size(vec2(ui.available_width(), 1.0), Sense::empty());

    ui.painter().hline(
        sep.x_range(),
        sep.center().y,
        Stroke::new(1.0, colors::DIVIDER),
    );
}
