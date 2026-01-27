use egui::{
    Align2, Button, Color32, Context, FontId, Frame, Pos2, Rect, RichText, Sense, Stroke, Ui, pos2,
    vec2,
};

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
