use super::*;

// show modal of downloading update, if it exists
// non closable until it finish
pub fn show(ctx: &Context, progress: &DownloadProgress) {
    base::Modal::new("updater_modal").closable(false).show(ctx, |ui| {
        ui.with_layout(Layout::top_down(Align::Center), |ui| {
            let title = format!("Downloading update v{}", progress.version);

            ui.label(RichText::new(title).size(18.0).color(colors::FG).strong());

            ui.add_space(20.0);

            let fraction = if progress.total > 0 {
                progress.downloaded as f32 / progress.total as f32
            } else {
                0.0
            };

            let bar = ProgressBar::new(fraction)
                .fill(colors::PRIMARY)
                .desired_width(400.0);

            ui.add(bar);
            ui.add_space(12.0);

            let downloaded_mb = progress.downloaded as f64 / 1_048_576.0;
            let total_mb = progress.total as f64 / 1_048_576.0;
            let percent = (fraction * 100.0) as u8;
            let text = format!("{:.1} / {:.1} MB ({}%)", downloaded_mb, total_mb, percent);

            ui.label(RichText::new(text).size(14.0).color(colors::FG_MUTED));
        });
    });
}
