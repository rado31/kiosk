use egui::{Align2, Button, Context, FontId, Frame, Rect, RichText, Sense, Stroke, Ui, pos2, vec2};

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
