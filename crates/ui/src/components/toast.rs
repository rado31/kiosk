use egui::{Color32, CornerRadius, Response, Shadow, Stroke, Ui, Vec2};
use egui_toast::{Toast, ToastKind, Toasts};

use crate::theme::{colors, corners};

const PADDING: f32 = 32.0;
const ACCENT_WIDTH: f32 = 8.0;
const PROGRESS_HEIGHT: f32 = 4.0;

fn accent_color(kind: ToastKind) -> Color32 {
    match kind {
        ToastKind::Info => colors::INFO,
        ToastKind::Warning => colors::WARNING,
        ToastKind::Error => colors::ERROR,
        ToastKind::Success => colors::SUCCESS,
        _ => colors::INFO,
    }
}

fn bg_color(kind: ToastKind) -> Color32 {
    match kind {
        ToastKind::Info => colors::INFO_BG,
        ToastKind::Warning => colors::WARNING_BG,
        ToastKind::Error => colors::ERROR_BG,
        ToastKind::Success => colors::SUCCESS_BG,
        _ => colors::INFO_BG,
    }
}

fn render(ui: &mut Ui, toast: &mut Toast) -> Response {
    let accent = accent_color(toast.kind);
    let tint = bg_color(toast.kind);

    let shadow = Shadow {
        offset: [0, 2],
        blur: 8,
        spread: 0,
        color: colors::SHADOW,
    };

    let frame = egui::Frame::new()
        .fill(tint)
        .shadow(shadow)
        .corner_radius(corners::MEDIUM)
        .stroke(Stroke::NONE)
        .inner_margin(egui::Margin {
            left: (PADDING + ACCENT_WIDTH) as i8,
            right: PADDING as i8,
            top: (PADDING / 2.0) as i8,
            bottom: (PADDING / 2.0) as i8,
        });

    let response = frame
        .show(ui, |ui| {
            ui.style_mut().override_text_style = Some(egui::TextStyle::Body);
            ui.visuals_mut().override_text_color = Some(colors::FG);
            ui.style_mut().override_font_id = Some(egui::FontId::proportional(28.0));
            ui.label(toast.text.clone());
        })
        .response;

    let rect = response.rect;
    let painter = ui.painter();

    // Accent bar on the left edge
    let accent_rect =
        egui::Rect::from_min_size(rect.left_top(), Vec2::new(ACCENT_WIDTH, rect.height()));

    let accent_rounding = CornerRadius {
        nw: corners::MEDIUM.nw,
        sw: corners::MEDIUM.sw,
        ne: 0,
        se: 0,
    };

    painter.rect_filled(accent_rect, accent_rounding, accent);

    // Progress bar at the bottom
    if toast.options.show_progress {
        let progress = toast.options.progress() as f32;
        let bar_rounding = CornerRadius {
            nw: 0,
            ne: 0,
            sw: corners::MEDIUM.sw,
            se: corners::MEDIUM.se,
        };

        let mut bar_rect = rect;

        bar_rect.set_top(bar_rect.bottom() - PROGRESS_HEIGHT);
        bar_rect.set_right(bar_rect.left() + bar_rect.width() * progress);
        painter.rect_filled(bar_rect, bar_rounding, accent.gamma_multiply(0.5));
    }

    response
}

pub fn create() -> Toasts {
    Toasts::new()
        .anchor(egui::Align2::CENTER_TOP, (0.0, 20.0))
        .direction(egui::Direction::TopDown)
        .custom_contents(ToastKind::Info, render)
        .custom_contents(ToastKind::Warning, render)
        .custom_contents(ToastKind::Error, render)
        .custom_contents(ToastKind::Success, render)
}
