use super::*;

mod left;
mod middle;
mod right;

pub const HEIGHT: i8 = 60;

pub fn show(ctx: &Context, state: &mut State) {
    let screen_width = ctx.input(|i| i.viewport_rect().width());

    Area::new(Id::new("header"))
        .fixed_pos(pos2(0.0, 0.0))
        .order(Order::Foreground)
        .show(ctx, |ui| {
            Frame::NONE
                .fill(colors::WHITE)
                .shadow(Shadow {
                    offset: [0, 2],
                    blur: 8,
                    spread: 0,
                    color: colors::SHADOW,
                })
                .inner_margin(Margin::symmetric(30, 10))
                .show(ui, |ui| {
                    ui.set_width(screen_width - 60.0);

                    let width = ui.available_width();

                    ui.horizontal(|ui| {
                        ui.set_height(40.0);

                        let frame = Frame::NONE;
                        let left_width = width * 0.2;
                        let right_width = width * 0.2;
                        let middle_width = width - left_width - right_width;

                        left::show(frame, ui, left_width);
                        middle::show(frame, ui, middle_width);
                        right::show(frame, ui, right_width, state);
                    });
                });
        });
}
