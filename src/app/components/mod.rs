use egui::{
    Align, Align2, Area, Button, Color32, Context, CornerRadius, Frame, Id, Image, ImageSource,
    Layout, Margin, Order, Pos2, ProgressBar, RichText, Sense, Stroke, StrokeKind, TopBottomPanel,
    Ui, UiBuilder, Vec2, include_image, vec2,
};

use crate::app::{
    constants::{colors, corners},
    i18n::t,
    services::updater::DownloadProgress,
    state::{Language, State},
    views::View,
};

pub mod base;
mod header;
mod menu;
pub mod toast;
pub mod updater_modal;

pub use header::Header;
pub use menu::Menu;
