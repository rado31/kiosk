use std::collections::BTreeMap;

use egui::{Align2, FontId, Rect, Sense, Stroke, StrokeKind, Ui, pos2, vec2};

use crate::{
    i18n::t,
    state::State,
    theme::{colors, corners},
};

const LABEL_W: f32 = 80.0;
const CELL_MIN: f32 = 24.0;
const H_GAP: f32 = 4.0;
const V_GAP: f32 = 8.0;
const COMP_GAP: f32 = 12.0;

/// Number of floors for a given wagon type ID.
///   1 → 1 floor   (sleeping)
///   2 → 2 floors  (kupe)
///   3 → 3 floors  (platzkart)
///   5 → 2 floors  (VIP kupe)
fn floor_count(wagon_type_id: u32) -> usize {
    match wagon_type_id {
        1 => 1,
        2 | 5 => 2,
        _ => 3,
    }
}

/// Maps seat label to `(floor_index_from_bottom, compartment_index)`.
/// Floor index 0 = lowest floor (floor 1).
fn seat_placement(label: u32, floors: usize) -> (usize, u32) {
    match floors {
        1 => (0, (label - 1) / 2),
        2 => {
            let floor = if label.is_multiple_of(2) { 1 } else { 0 };
            let comp = (label - 1) / 4;

            (floor, comp)
        }
        _ => {
            let floor = match label % 3 {
                1 => 0,
                2 => 1,
                _ => 2, // label % 3 == 0
            };

            let comp = (label - 1) / 6;

            (floor, comp)
        }
    }
}

/// Organizes seats into `Vec<compartment>` where each compartment is
/// `Vec<floor_seats>` ordered top→bottom (highest floor first).
/// Each `floor_seats` is sorted by label ascending.
fn organize_seats(seats: &[api::trips::Seat], floors: usize) -> Vec<Vec<Vec<api::trips::Seat>>> {
    let mut by_comp: BTreeMap<u32, Vec<Vec<api::trips::Seat>>> = BTreeMap::new();

    for seat in seats {
        if let Ok(label) = seat.label.parse::<u32>() {
            let (floor_idx, comp_idx) = seat_placement(label, floors);
            let entry = by_comp
                .entry(comp_idx)
                .or_insert_with(|| (0..floors).map(|_| Vec::new()).collect());

            entry[floor_idx].push(seat.clone());
        }
    }

    let mut result: Vec<_> = by_comp.into_values().collect();

    for comp in &mut result {
        for floor_seats in comp.iter_mut() {
            floor_seats.sort_by_key(|s| s.label.parse::<u32>().unwrap_or(0));
        }

        comp.reverse(); // Reverse so highest floor is at index 0 (rendered top)
    }

    result
}

pub(super) fn render(state: &mut State, ui: &mut Ui) {
    let wagon_idx = state.seats.current_wagon;
    let Some(wagon) = state.seats.current_wagon().cloned() else {
        return;
    };

    let floors = floor_count(wagon.wagon_type_id);
    let compartments = organize_seats(&wagon.seats, floors);

    if compartments.is_empty() {
        return;
    }

    let n_comps = compartments.len();
    let n = n_comps as f32;
    let available_w = ui.available_width();

    let cell_s = ((available_w - LABEL_W - n * H_GAP - (n - 1.0).max(0.0) * COMP_GAP) / (n * 2.0))
        .max(CELL_MIN);

    let comp_w = cell_s * 2.0 + H_GAP;
    let total_grid_w = LABEL_W + n * comp_w + (n - 1.0).max(0.0) * COMP_GAP;
    let grid_h = cell_s * floors as f32 + V_GAP * floors.saturating_sub(1) as f32;
    let font_size = (cell_s * 0.38).clamp(10.0, 18.0);

    let anim_id = ui.id().with("wagon_slide");
    let (grid_rect, _) = ui.allocate_exact_size(vec2(available_w, grid_h), Sense::empty());

    let slide_dir = std::mem::take(&mut state.seats.slide_dir);

    if slide_dir != 0.0 {
        let now = ui.input(|i| i.time);

        ui.ctx()
            .memory_mut(|m| m.data.insert_temp(anim_id, (slide_dir, now)));
    }

    // --- Compute visual x offset ---
    let (anim_dir, anim_t0) = ui
        .ctx()
        .memory(|m| m.data.get_temp::<(f32, f64)>(anim_id))
        .unwrap_or((0.0, 0.0));

    let offset_x = if anim_dir == 0.0 {
        0.0
    } else {
        let elapsed = ui.input(|i| i.time) - anim_t0;
        let progress = (elapsed / 0.25).min(1.0) as f32;
        let ease = 1.0 - (1.0 - progress).powi(2); // ease-out quadratic
        let val = anim_dir * available_w * (1.0 - ease);

        if val.abs() > 0.1 {
            ui.ctx().request_repaint();
        }

        val
    };

    // --- Paint grid, clipped to the viewport, shifted by offset_x ---
    let clip_rect = grid_rect;
    let grid_rect = Rect::from_min_size(
        pos2(grid_rect.min.x + offset_x, grid_rect.min.y),
        vec2(total_grid_w, grid_h),
    );

    let painter = ui.painter().with_clip_rect(clip_rect);

    // Floor labels — row 0 = top = floor N, row (floors-1) = bottom = floor 1
    for row_idx in 0..floors {
        let floor_num = floors - row_idx;
        let key = format!("floor_{}", floor_num);
        let label_y = grid_rect.min.y + row_idx as f32 * (cell_s + V_GAP) + cell_s / 2.0;

        painter.text(
            pos2(grid_rect.min.x + LABEL_W - 8.0, label_y),
            Align2::RIGHT_CENTER,
            t(&state.lang, &key),
            FontId::proportional(font_size),
            colors::FG_MUTED,
        );
    }

    let mut clicked: Option<(usize, u32, String)> = None;
    let mut unassign: Option<(usize, u32)> = None;

    for (comp_idx, comp) in compartments.iter().enumerate() {
        let comp_x = grid_rect.min.x + LABEL_W + comp_idx as f32 * (comp_w + COMP_GAP);

        if comp_idx > 0 {
            let sep_x = comp_x - COMP_GAP / 2.0;

            painter.vline(
                sep_x,
                grid_rect.min.y..=grid_rect.max.y,
                Stroke::new(2.0, colors::BLACK),
            );
        }

        for (row_idx, floor_seats) in comp.iter().enumerate() {
            let cell_y = grid_rect.min.y + row_idx as f32 * (cell_s + V_GAP);

            for (cell_idx, seat) in floor_seats.iter().enumerate().take(2) {
                let cell_x = comp_x + cell_idx as f32 * (cell_s + H_GAP);
                let cell_rect = Rect::from_min_size(pos2(cell_x, cell_y), vec2(cell_s, cell_s));
                let is_selected = state.seats.is_seat_selected(wagon_idx, seat.id);

                let (bg, fg, text) = if is_selected {
                    (colors::PRIMARY, colors::WHITE, seat.label.clone())
                } else if !seat.available {
                    (colors::BG_5, colors::FG_DISABLED, "x".to_string())
                } else {
                    (colors::WHITE, colors::FG, seat.label.clone())
                };

                if seat.available && !is_selected {
                    painter.rect(
                        cell_rect,
                        corners::SMALL,
                        bg,
                        Stroke::new(1.0, colors::BORDER),
                        StrokeKind::Inside,
                    );
                } else {
                    painter.rect_filled(cell_rect, corners::SMALL, bg);
                }

                painter.text(
                    cell_rect.center(),
                    Align2::CENTER_CENTER,
                    &text,
                    FontId::proportional(font_size),
                    fg,
                );

                if seat.available || is_selected {
                    let res = ui.interact(cell_rect, ui.id().with(("seat", seat.id)), Sense::CLICK);

                    if res.clicked() {
                        if is_selected {
                            unassign = Some((wagon_idx, seat.id));
                        } else {
                            clicked = Some((wagon_idx, seat.id, seat.label.clone()));
                        }
                    }
                }
            }
        }
    }

    if let Some((widx, sid)) = unassign {
        state.seats.unassign_seat(widx, sid);
    } else if let Some((widx, id, label)) = clicked {
        state.seats.assign_seat(widx, id, label);
    }
}
