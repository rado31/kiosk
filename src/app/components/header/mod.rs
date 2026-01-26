use super::*;

mod left;
mod middle;
mod right;

pub fn show(ctx: &Context, state: &mut State) {
    let frame = Frame::new()
        .fill(colors::WHITE)
        .stroke(Stroke::new(1.0, colors::BORDER))
        .inner_margin(Margin::symmetric(30, 10));

    let top_panel = TopBottomPanel::top("top_panel")
        .frame(frame)
        .show_separator_line(false);

    top_panel.show(ctx, |ui| {
        ui.columns_const(|[col1, col2, col3]| {
            col1.horizontal_centered(|ui| left::show(ui));
            col2.centered_and_justified(|ui| middle::show(ui));
            col3.with_layout(Layout::right_to_left(Align::Center), |ui| {
                right::show(ui, state)
            });
        });
    });
}
