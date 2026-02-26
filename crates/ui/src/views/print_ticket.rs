use egui::{
    Align2, Button, Color32, FontFamily, FontId, Frame, Id, Pos2, Rect, RichText, Sense, Shadow,
    Stroke, StrokeKind, Ui, pos2, vec2,
};

use crate::{
    components::keyboard,
    i18n::t,
    state::{State, print_ticket::TicketSource},
    theme::{colors, corners},
};

const DISPLAY_WIDTH: f32 = 200.0;
const DISPLAY_HEIGHT: f32 = 60.0;
const DISPLAY_FONT_SIZE: f32 = 32.0;
const TAB_FONT_SIZE: f32 = 17.0;

pub fn show(state: &mut State, ui: &mut Ui) {
    let ctx = ui.ctx().clone();

    let card = Frame::new()
        .inner_margin(32)
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

        ui.vertical_centered(|ui| {
            render_ticket_source_toggle(state, ui);
            ui.add_space(30.0);

            match state.print_ticket.source {
                TicketSource::Terminal => render_terminal_tab(state, ui),
                TicketSource::External => render_online_tab(state, ui),
            }
        });
    });

    match state.print_ticket.source {
        TicketSource::Terminal => keyboard::show(
            &mut state.keyboard_visible,
            &mut state.print_ticket.terminal_code,
            Some(6),
            &ctx,
            Id::new("kb_terminal"),
        ),
        TicketSource::External => keyboard::show(
            &mut state.keyboard_visible,
            &mut state.print_ticket.online_code,
            Some(6),
            &ctx,
            Id::new("kb_online"),
        ),
    };
}

fn render_ticket_source_toggle(state: &mut State, ui: &mut Ui) {
    const BTN_WIDTH: f32 = 300.0;
    const BTN_HEIGHT: f32 = 50.0;
    const PADDING: f32 = 5.0;

    let total = vec2(BTN_WIDTH * 2.0 + PADDING * 2.0, BTN_HEIGHT + PADDING * 2.0);
    let (outer_rect, _) = ui.allocate_exact_size(total, Sense::empty());

    let shadow = Shadow {
        offset: [0, 2],
        blur: 8,
        spread: 0,
        color: colors::SHADOW,
    };

    ui.painter()
        .add(shadow.as_shape(outer_rect, corners::MEDIUM));
    ui.painter()
        .rect_filled(outer_rect, corners::MEDIUM, colors::WHITE);

    let rect1 = Rect::from_min_size(
        pos2(outer_rect.min.x + PADDING, outer_rect.min.y + PADDING),
        vec2(BTN_WIDTH, BTN_HEIGHT),
    );

    let rect2 = Rect::from_min_size(
        pos2(
            outer_rect.min.x + PADDING + BTN_WIDTH,
            outer_rect.min.y + PADDING,
        ),
        vec2(BTN_WIDTH, BTN_HEIGHT),
    );

    let res1 = ui.interact(rect1, ui.id().with("tab_terminal"), Sense::CLICK);
    let res2 = ui.interact(rect2, ui.id().with("tab_online"), Sense::CLICK);

    if res1.clicked() {
        state.print_ticket.source = TicketSource::Terminal;
        state.keyboard_visible = false;
    }

    if res2.clicked() {
        state.print_ticket.source = TicketSource::External;
        state.keyboard_visible = false;
    }

    let is_terminal = state.print_ticket.source == TicketSource::Terminal;
    let curr_rect = if is_terminal { rect1 } else { rect2 };

    let anime_x = ui.ctx().animate_value_with_time(
        ui.id().with("ticket_source_tab_anim"),
        curr_rect.min.x,
        0.2,
    );

    let indicator = Rect::from_min_size(pos2(anime_x, curr_rect.min.y), curr_rect.size());

    ui.painter()
        .rect_filled(indicator, corners::MEDIUM, colors::BTN_PRIMARY_BG);

    let (c1, c2) = if is_terminal {
        (colors::WHITE, colors::FG)
    } else {
        (colors::FG, colors::WHITE)
    };

    draw_tab_text(ui, rect1.center(), c1, t(&state.lang, "terminal_ticket"));
    draw_tab_text(ui, rect2.center(), c2, t(&state.lang, "external_ticket"));
}

fn render_terminal_tab(state: &mut State, ui: &mut Ui) {
    let title = RichText::new(t(&state.lang, "enter_booking_number"))
        .size(24.0)
        .family(FontFamily::Name("bold".into()))
        .color(colors::FG);

    ui.label(title);
    ui.add_space(30.0);

    render_code_input(
        ui,
        &state.print_ticket.terminal_code,
        &mut state.keyboard_visible,
    );

    ui.add_space(30.0);

    let btn_label = RichText::new(t(&state.lang, "print"))
        .size(20.0)
        .family(FontFamily::Name("bold".into()))
        .color(colors::WHITE);

    let btn = Button::new(btn_label)
        .min_size(vec2(200.0, 60.0))
        .fill(colors::BTN_PRIMARY_BG)
        .corner_radius(12);

    if ui.add(btn).clicked() {
        log::debug!(
            "print terminal ticket: {}",
            state.print_ticket.terminal_code
        );
    }
}

fn render_online_tab(state: &mut State, ui: &mut Ui) {
    let title = RichText::new(t(&state.lang, "enter_booking_number"))
        .size(24.0)
        .family(FontFamily::Name("bold".into()))
        .color(colors::FG);

    ui.label(title);
    ui.add_space(30.0);

    render_code_input(
        ui,
        &state.print_ticket.online_code,
        &mut state.keyboard_visible,
    );

    ui.add_space(20.0);

    let notice = RichText::new(t(&state.lang, "external_print_notice"))
        .size(16.0)
        .color(colors::FG_MUTED);

    ui.label(notice);
    ui.add_space(20.0);

    let btn_label = RichText::new(t(&state.lang, "pay_and_print"))
        .size(20.0)
        .family(FontFamily::Name("bold".into()))
        .color(colors::WHITE);

    let btn = Button::new(btn_label)
        .min_size(vec2(200.0, 60.0))
        .fill(colors::BTN_GREEN)
        .corner_radius(12);

    if ui.add(btn).clicked() {
        log::debug!(
            "pay and print online ticket: {}",
            state.print_ticket.online_code
        );
    }
}

fn render_code_input(ui: &mut Ui, code: &str, keyboard_visible: &mut bool) {
    let (rect, response) =
        ui.allocate_exact_size(vec2(DISPLAY_WIDTH, DISPLAY_HEIGHT), Sense::CLICK);

    let border_color = if *keyboard_visible {
        colors::PRIMARY
    } else {
        colors::BORDER
    };

    ui.painter().rect(
        rect,
        corners::MEDIUM,
        colors::INPUT_BG,
        Stroke::new(1.0, border_color),
        StrokeKind::Inside,
    );

    let (text, text_color) = if code.is_empty() {
        ("XXXXXX", colors::FG_PLACEHOLDER)
    } else {
        (code, colors::FG)
    };

    ui.painter().text(
        rect.center(),
        Align2::CENTER_CENTER,
        text,
        FontId::new(DISPLAY_FONT_SIZE, FontFamily::Proportional),
        text_color,
    );

    if response.clicked() {
        *keyboard_visible = true;
    }
}

fn draw_tab_text(ui: &Ui, center: Pos2, color: Color32, text: &str) {
    ui.painter().text(
        center,
        Align2::CENTER_CENTER,
        text,
        FontId::new(TAB_FONT_SIZE, FontFamily::Name("bold".into())),
        color,
    );
}
