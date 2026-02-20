use egui::{
    Align, Align2, Area, Context, FontFamily, FontId, Frame, Id, Image, Margin, Order, Rect, Sense,
    Shadow, Ui, Vec2, include_image,
    text::{CCursor, CCursorRange},
    text_edit::TextEditState,
    vec2,
};

use crate::{
    components::input::Input,
    theme::{colors, corners},
};

// ── Shared layout constants ───────────────────────────────────────────────────
const INPUT_HEIGHT: f32 = 60.0;
const INPUT_FONT_SIZE: f32 = 32.0;
const INPUT_GAP: f32 = 20.0;
const KEY_GAP: f32 = 8.0;
const ROW_GAP: f32 = 10.0;
const H_PADDING: f32 = 20.0;
const V_PADDING: f32 = 20.0;

// ── Full keyboard ─────────────────────────────────────────────────────────────
const KB_INPUT_ID: &str = "keyboard_input";
const KEY_HEIGHT: f32 = 60.0;
const KEY_FONT_SIZE: f32 = 22.0;

const ROW_NUMS: &[char] = &['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];
const ROW0: &[char] = &['Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P'];
const ROW1: &[char] = &['A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L'];
const ROW2: &[char] = &['Z', 'X', 'C', 'V', 'B', 'N', 'M'];

// ── Numpad ────────────────────────────────────────────────────────────────────
#[allow(dead_code)]
const NP_INPUT_ID: &str = "numpad_input";
#[allow(dead_code)]
const NP_KEY_WIDTH: f32 = 106.0;
#[allow(dead_code)]
const NP_KEY_HEIGHT: f32 = 80.0;
#[allow(dead_code)]
const NP_KEY_FONT_SIZE: f32 = 32.0;
#[allow(dead_code)]
const NP_KEY_GAP: f32 = 10.0;
#[allow(dead_code)]
const NP_ROW_GAP: f32 = 10.0;

// ─────────────────────────────────────────────────────────────────────────────

pub fn show(visible: &mut bool, value: &mut String, max_len: Option<usize>, ctx: &Context) {
    if !*visible {
        return;
    }

    let screen_rect = ctx.input(|i| i.viewport_rect());

    // Invisible full-screen overlay — clicking outside the keyboard closes it
    Area::new(Id::new("keyboard_overlay"))
        .order(Order::Foreground)
        .fixed_pos(screen_rect.min)
        .show(ctx, |ui| {
            ui.expand_to_include_rect(screen_rect);

            let response = ui.allocate_response(screen_rect.size(), Sense::CLICK);

            if response.clicked() {
                *visible = false;
            }
        });

    // Keyboard panel anchored to the bottom
    Area::new(Id::new("keyboard_panel"))
        .order(Order::Tooltip)
        .anchor(Align2::LEFT_BOTTOM, Vec2::ZERO)
        .show(ctx, |ui| {
            let screen_width = screen_rect.width();
            let panel = Frame::new()
                .fill(colors::WHITE)
                .shadow(Shadow {
                    offset: [0, -4],
                    blur: 20,
                    spread: 0,
                    color: colors::SHADOW,
                })
                .inner_margin(Margin::symmetric(H_PADDING as i8, V_PADDING as i8));

            panel.show(ui, |ui| {
                ui.set_width(screen_width);
                ui.spacing_mut().item_spacing = Vec2::ZERO;

                let available = screen_width - 2.0 * H_PADDING;

                // Input at the top — the only interactive input
                let mut input = Input::new(value)
                    .id(Id::new(KB_INPUT_ID))
                    .font_size(INPUT_FONT_SIZE)
                    .horizontal_align(Align::Center)
                    .desired_size(vec2(available - 24.0, INPUT_HEIGHT));

                if let Some(max) = max_len {
                    input = input.char_limit(max);
                }

                input.show(ui);
                ui.add_space(INPUT_GAP);

                // Key rows
                let key_width = (available - 9.0 * KEY_GAP) / 10.0;

                render_letter_row(ui, ctx, ROW_NUMS, key_width, 0.0, value, max_len);
                ui.add_space(ROW_GAP);

                render_letter_row(ui, ctx, ROW0, key_width, 0.0, value, max_len);
                ui.add_space(ROW_GAP);

                let indent = (key_width + KEY_GAP) / 2.0;
                render_letter_row(ui, ctx, ROW1, key_width, indent, value, max_len);
                ui.add_space(ROW_GAP);

                render_row2(ui, ctx, key_width, available, value, max_len);
                ui.add_space(ROW_GAP);

                render_space(ui, ctx, available, value, max_len);
            });
        });
}

#[allow(dead_code)]
pub fn show_numpad(visible: &mut bool, value: &mut String, max_len: Option<usize>, ctx: &Context) {
    if !*visible {
        return;
    }

    let screen_rect = ctx.input(|i| i.viewport_rect());

    // Invisible full-screen overlay — clicking outside closes it
    Area::new(Id::new("numpad_overlay"))
        .order(Order::Foreground)
        .fixed_pos(screen_rect.min)
        .show(ctx, |ui| {
            ui.expand_to_include_rect(screen_rect);
            let response = ui.allocate_response(screen_rect.size(), Sense::CLICK);
            if response.clicked() {
                *visible = false;
            }
        });

    // Numpad panel — full width, keys centered
    Area::new(Id::new("numpad_panel"))
        .order(Order::Tooltip)
        .anchor(Align2::LEFT_BOTTOM, Vec2::ZERO)
        .show(ctx, |ui| {
            let screen_width = screen_rect.width();
            let panel = Frame::new()
                .fill(colors::WHITE)
                .shadow(Shadow {
                    offset: [0, -4],
                    blur: 20,
                    spread: 0,
                    color: colors::SHADOW,
                })
                .inner_margin(Margin::symmetric(H_PADDING as i8, V_PADDING as i8));

            panel.show(ui, |ui| {
                ui.set_width(screen_width);
                ui.spacing_mut().item_spacing = Vec2::ZERO;

                let available = screen_width - 2.0 * H_PADDING;
                let grid_width = 3.0 * NP_KEY_WIDTH + 2.0 * NP_KEY_GAP;
                let offset = (available - grid_width) / 2.0;

                // Centered input (same width as the key grid)
                ui.horizontal(|ui| {
                    ui.add_space(offset);

                    let mut input = Input::new(value)
                        .id(Id::new(NP_INPUT_ID))
                        .font_size(INPUT_FONT_SIZE)
                        .horizontal_align(Align::Center)
                        .desired_size(vec2(grid_width - 24.0, INPUT_HEIGHT));

                    if let Some(max) = max_len {
                        input = input.char_limit(max);
                    }

                    input.show(ui);
                });

                ui.add_space(INPUT_GAP);

                for row in [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']] {
                    render_numpad_row(ui, ctx, &row, offset, value, max_len);
                    ui.add_space(NP_ROW_GAP);
                }

                render_numpad_bottom(ui, ctx, offset, value, max_len);
            });
        });
}

// ── Full keyboard helpers ─────────────────────────────────────────────────────

fn render_letter_row(
    ui: &mut Ui,
    ctx: &Context,
    keys: &[char],
    key_width: f32,
    indent: f32,
    value: &mut String,
    max_len: Option<usize>,
) {
    ui.horizontal(|ui| {
        if indent > 0.0 {
            ui.add_space(indent);
        }

        for (i, &ch) in keys.iter().enumerate() {
            if i > 0 {
                ui.add_space(KEY_GAP);
            }

            if key_tapped(ui, key_width, KEY_HEIGHT, KEY_FONT_SIZE, &ch.to_string()) {
                insert_at_cursor(ctx, Id::new(KB_INPUT_ID), value, ch, max_len);
            }
        }
    });
}

fn render_row2(
    ui: &mut Ui,
    ctx: &Context,
    key_width: f32,
    available: f32,
    value: &mut String,
    max_len: Option<usize>,
) {
    let backspace_width = available - 7.0 * key_width - 7.0 * KEY_GAP;

    ui.horizontal(|ui| {
        for (i, &ch) in ROW2.iter().enumerate() {
            if i > 0 {
                ui.add_space(KEY_GAP);
            }

            if key_tapped(ui, key_width, KEY_HEIGHT, KEY_FONT_SIZE, &ch.to_string()) {
                insert_at_cursor(ctx, Id::new(KB_INPUT_ID), value, ch, max_len);
            }
        }

        ui.add_space(KEY_GAP);

        if backspace_tapped(ui, backspace_width, KEY_HEIGHT) {
            backspace_at_cursor(ctx, Id::new(KB_INPUT_ID), value);
        }
    });
}

fn render_space(
    ui: &mut Ui,
    ctx: &Context,
    available: f32,
    value: &mut String,
    max_len: Option<usize>,
) {
    ui.horizontal(|ui| {
        let (rect, response) = ui.allocate_exact_size(vec2(available, KEY_HEIGHT), Sense::CLICK);

        let fill = if response.is_pointer_button_down_on() {
            colors::BG_5
        } else if response.hovered() {
            colors::BG_4
        } else {
            colors::BG_DIM
        };

        ui.painter().rect_filled(rect, corners::SMALL, fill);

        let icon_size = vec2(32.0, 32.0);
        let icon_rect = Rect::from_center_size(rect.center(), icon_size);

        ui.put(
            icon_rect,
            Image::new(include_image!("../assets/space.svg"))
                .fit_to_exact_size(icon_size)
                .tint(colors::FG),
        );

        if response.clicked() {
            insert_at_cursor(ctx, Id::new(KB_INPUT_ID), value, ' ', max_len);
        }
    });
}

// ── Numpad helpers ────────────────────────────────────────────────────────────

#[allow(dead_code)]
fn render_numpad_row(
    ui: &mut Ui,
    ctx: &Context,
    keys: &[char; 3],
    offset: f32,
    value: &mut String,
    max_len: Option<usize>,
) {
    ui.horizontal(|ui| {
        ui.add_space(offset);
        for (i, &ch) in keys.iter().enumerate() {
            if i > 0 {
                ui.add_space(NP_KEY_GAP);
            }

            if key_tapped(
                ui,
                NP_KEY_WIDTH,
                NP_KEY_HEIGHT,
                NP_KEY_FONT_SIZE,
                &ch.to_string(),
            ) {
                insert_at_cursor(ctx, Id::new(NP_INPUT_ID), value, ch, max_len);
            }
        }
    });
}

#[allow(dead_code)]
fn render_numpad_bottom(
    ui: &mut Ui,
    ctx: &Context,
    offset: f32,
    value: &mut String,
    max_len: Option<usize>,
) {
    ui.horizontal(|ui| {
        ui.add_space(offset);

        // Empty slot — keeps 0 and ⌫ aligned to the right two columns
        ui.add_space(NP_KEY_WIDTH);
        ui.add_space(NP_KEY_GAP);

        // 0
        if key_tapped(ui, NP_KEY_WIDTH, NP_KEY_HEIGHT, NP_KEY_FONT_SIZE, "0") {
            insert_at_cursor(ctx, Id::new(NP_INPUT_ID), value, '0', max_len);
        }

        ui.add_space(NP_KEY_GAP);

        // Backspace
        if backspace_tapped(ui, NP_KEY_WIDTH, NP_KEY_HEIGHT) {
            backspace_at_cursor(ctx, Id::new(NP_INPUT_ID), value);
        }
    });
}

// ── Shared primitives ─────────────────────────────────────────────────────────

/// Renders a single key and returns `true` if clicked.
fn key_tapped(ui: &mut Ui, width: f32, height: f32, font_size: f32, label: &str) -> bool {
    let (rect, response) = ui.allocate_exact_size(vec2(width, height), Sense::CLICK);

    let fill = if response.is_pointer_button_down_on() {
        colors::BG_5
    } else if response.hovered() {
        colors::BG_4
    } else {
        colors::BG_DIM
    };

    ui.painter().rect_filled(rect, corners::SMALL, fill);
    ui.painter().text(
        rect.center(),
        Align2::CENTER_CENTER,
        label,
        FontId::new(font_size, FontFamily::Proportional),
        colors::FG,
    );

    response.clicked()
}

/// Renders a backspace key and returns `true` if clicked.
fn backspace_tapped(ui: &mut Ui, width: f32, height: f32) -> bool {
    let (rect, response) = ui.allocate_exact_size(vec2(width, height), Sense::CLICK);

    let fill = if response.is_pointer_button_down_on() {
        colors::BG_5
    } else if response.hovered() {
        colors::BG_4
    } else {
        colors::BG_DIM
    };

    ui.painter().rect_filled(rect, corners::SMALL, fill);

    let icon_size = vec2(28.0, 28.0);
    let icon_rect = Rect::from_center_size(rect.center(), icon_size);

    ui.put(
        icon_rect,
        Image::new(include_image!("../assets/backspace.svg"))
            .fit_to_exact_size(icon_size)
            .tint(colors::ERROR),
    );

    response.clicked()
}

/// Inserts a character at the cursor position and advances the cursor.
/// Falls back to appending if TextEditState is unavailable.
fn insert_at_cursor(ctx: &Context, id: Id, value: &mut String, ch: char, max_len: Option<usize>) {
    if max_len.is_some_and(|m| value.len() >= m) {
        return;
    }

    let cursor_pos = TextEditState::load(ctx, id)
        .and_then(|s| s.cursor.char_range())
        .map(|r| r.primary.index);

    let pos = match cursor_pos {
        Some(p) => p,
        None => {
            value.push(ch);
            return;
        }
    };

    let byte_pos = value
        .char_indices()
        .nth(pos)
        .map(|(i, _)| i)
        .unwrap_or(value.len());

    value.insert(byte_pos, ch);

    if let Some(mut state) = TextEditState::load(ctx, id) {
        let new_cursor = CCursorRange::one(CCursor::new(pos + 1));

        state.cursor.set_char_range(Some(new_cursor));
        state.store(ctx, id);
    }
}

/// Deletes the character before the cursor and moves the cursor back.
/// Falls back to popping the last char if TextEditState is unavailable.
fn backspace_at_cursor(ctx: &Context, id: Id, value: &mut String) {
    let cursor_pos = TextEditState::load(ctx, id)
        .and_then(|s| s.cursor.char_range())
        .map(|r| r.primary.index);

    let pos = match cursor_pos {
        Some(p) => p,
        None => {
            value.pop();
            return;
        }
    };

    if pos == 0 || value.is_empty() {
        return;
    }

    let byte_pos = value.char_indices().nth(pos - 1).map(|(i, _)| i);

    if let Some(byte_i) = byte_pos {
        value.remove(byte_i);

        if let Some(mut state) = TextEditState::load(ctx, id) {
            let new_cursor = CCursorRange::one(CCursor::new(pos - 1));

            state.cursor.set_char_range(Some(new_cursor));
            state.store(ctx, id);
        }
    }
}
