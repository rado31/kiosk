use egui::{Align2, FontFamily, FontId, Rect, Sense, Shadow, Ui, pos2, vec2};

use crate::{
    i18n::t,
    state::State,
    theme::{colors, corners},
};

pub(super) fn render(state: &mut State, ui: &mut Ui) {
    let total_w = ui.available_width();
    let nav_h = 44.0;
    let btn_w = 44.0;

    let (nav_rect, _) = ui.allocate_exact_size(vec2(total_w, nav_h), Sense::empty());

    let prev_rect = Rect::from_min_size(nav_rect.min, vec2(btn_w, nav_h));
    let next_rect = Rect::from_min_size(
        pos2(nav_rect.max.x - btn_w, nav_rect.min.y),
        vec2(btn_w, nav_h),
    );

    let can_prev = state.seats.current_wagon > 0;
    let can_next = state.seats.current_wagon < state.seats.wagon_count().saturating_sub(1);

    let shadow = Shadow {
        offset: [0, 2],
        blur: 8,
        spread: 0,
        color: colors::SHADOW,
    };

    for (rect, enabled) in [(prev_rect, can_prev), (next_rect, can_next)] {
        let bg = if enabled { colors::WHITE } else { colors::BG_5 };
        ui.painter().add(shadow.as_shape(rect, corners::MEDIUM));
        ui.painter().rect_filled(rect, corners::MEDIUM, bg);
    }

    let fg_prev = if can_prev {
        colors::FG
    } else {
        colors::FG_DISABLED
    };
    let fg_next = if can_next {
        colors::FG
    } else {
        colors::FG_DISABLED
    };

    ui.painter().text(
        prev_rect.center(),
        Align2::CENTER_CENTER,
        "←",
        FontId::proportional(18.0),
        fg_prev,
    );

    ui.painter().text(
        next_rect.center(),
        Align2::CENTER_CENTER,
        "→",
        FontId::proportional(18.0),
        fg_next,
    );

    let wagon_num = state.seats.current_wagon().map_or(0, |w| w.number);
    let wagon_text = format!("{} {}", t(&state.lang, "wagon"), wagon_num);

    ui.painter().text(
        nav_rect.center(),
        Align2::CENTER_CENTER,
        &wagon_text,
        FontId::new(18.0, FontFamily::Name("bold".into())),
        colors::FG,
    );

    let prev_res = ui.interact(prev_rect, ui.id().with("wagon_prev"), Sense::CLICK);
    let next_res = ui.interact(next_rect, ui.id().with("wagon_next"), Sense::CLICK);

    if can_prev && prev_res.clicked() {
        state.seats.prev_wagon();
    }

    if can_next && next_res.clicked() {
        state.seats.next_wagon();
    }
}
