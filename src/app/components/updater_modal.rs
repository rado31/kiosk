use egui::{Align, Context, Layout, ProgressBar, RichText, Ui};

use crate::app::{components::base::Modal, constants::colors, updater::DownloadProgress};

/// Show modal of downloading update, if it exists.
/// Non closable until it finish
pub fn open(ctx: &Context, progress: &DownloadProgress) {
    Modal::new("updater_modal")
        .closable(false)
        .open(ctx, |ui| render(ui, progress));
}

fn render(ui: &mut Ui, progress: &DownloadProgress) {
    ui.with_layout(Layout::top_down(Align::Center), |ui| {
        let title = format!("Downloading update {}", progress.version);

        ui.label(RichText::new(title).size(18.0).color(colors::FG));
        ui.add_space(20.0);

        let fraction = if progress.total > 0 {
            progress.downloaded as f32 / progress.total as f32
        } else {
            0.0
        };

        let bar = ProgressBar::new(fraction).fill(colors::PRIMARY);

        ui.add(bar);
        ui.add_space(12.0);

        const MB: f32 = 1_048_576.0;

        let downloaded_mb = progress.downloaded as f32 / MB;
        let total_mb = progress.total as f32 / MB;
        let percent = (fraction * 100.0) as u8;
        let text = format!("{:.1} / {:.1} MB ({}%)", downloaded_mb, total_mb, percent);

        ui.label(RichText::new(text).size(14.0).color(colors::FG_MUTED));
    });
}
