use egui::{Align2, FontId, RichText, Sense, Stroke, StrokeKind, Ui, vec2};

use crate::{
    i18n::t,
    state::State,
    theme::{colors, corners},
};

pub(super) fn render(state: &State, ui: &mut Ui) {
    const BOX_SIZE: f32 = 28.0;

    ui.horizontal(|ui| {
        ui.set_height(30.0);

        let explanation_lbl = RichText::new(format!("{} →", t(&state.lang, "explanation")))
            .size(15.0)
            .color(colors::FG_MUTED);

        ui.label(explanation_lbl);
        ui.add_space(16.0);

        // Free seat
        let (rect, _) = ui.allocate_exact_size(vec2(BOX_SIZE, BOX_SIZE), Sense::empty());

        ui.painter().rect(
            rect,
            corners::SMALL,
            colors::WHITE,
            Stroke::new(1.0, colors::BORDER),
            StrokeKind::Inside,
        );

        ui.add_space(4.0);

        let free_seats_lbl = RichText::new(t(&state.lang, "free_seats"))
            .size(15.0)
            .color(colors::FG);

        ui.label(free_seats_lbl);
        ui.add_space(20.0);

        // Taken seat
        let (rect, _) = ui.allocate_exact_size(vec2(BOX_SIZE, BOX_SIZE), Sense::empty());

        ui.painter().rect_filled(rect, corners::SMALL, colors::BG_5);
        ui.painter().text(
            rect.center(),
            Align2::CENTER_CENTER,
            "x",
            FontId::proportional(13.0),
            colors::FG_DISABLED,
        );

        ui.add_space(4.0);

        let taken_seats_lbl = RichText::new(t(&state.lang, "taken_seats"))
            .size(15.0)
            .color(colors::FG);

        ui.label(taken_seats_lbl);
        ui.add_space(20.0);

        // Selected seat
        let (rect, _) = ui.allocate_exact_size(vec2(BOX_SIZE, BOX_SIZE), Sense::empty());

        ui.painter()
            .rect_filled(rect, corners::SMALL, colors::PRIMARY);

        ui.add_space(4.0);

        let your_seats_lbl = RichText::new(t(&state.lang, "your_seats"))
            .size(15.0)
            .color(colors::FG);

        ui.label(your_seats_lbl);
    });
}
