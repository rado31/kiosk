use egui::{
    Align, Align2, Button, Color32, Context, FontId, Frame, Layout, Pos2, Rect, RichText, Sense,
    Stroke, StrokeKind, Ui, pos2, vec2,
};

use crate::debug;

use super::{
    State,
    components::base::Modal,
    constants::{colors, corners},
    i18n::t,
};

mod history;
mod home;
pub mod print_ticket;
pub mod refund;
pub mod seats;

pub use history::History;
pub use home::Home;
