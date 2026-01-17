use super::*;

mod left;
mod middle;
mod right;

pub fn show(ctx: &Context) {
    TopBottomPanel::top("header")
        .show_separator_line(false)
        .frame(
            Frame::NONE
                .fill(Color32::WHITE)
                .inner_margin(Margin::symmetric(30, 10)),
        )
        .show(ctx, |ui| {
            let frame = Frame::NONE;
            let width = ui.available_width(); // width of panel

            ui.horizontal(|ui| {
                ui.set_height(40.0);

                let left_width = width * 0.2;
                let right_width = width * 0.2;
                let middle_width = width - left_width - right_width;

                left::show(frame, ui, left_width);
                middle::show(frame, ui, middle_width);
                right::show(frame, ui, right_width);
            });
        });
}
