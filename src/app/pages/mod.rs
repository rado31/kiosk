use egui::{Align2, Context, FontId, Frame, Rect, Sense, Stroke, StrokeKind, Ui, pos2, vec2};

use super::{
    State,
    components::base::Modal,
    constants::{colors, corners},
    i18n::t,
};

mod home;
pub mod print_ticket;
pub mod refund;
pub mod seats;

pub use home::Home;
