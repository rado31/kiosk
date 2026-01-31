use egui::{
    Align, Align2, Area, Button, Color32, Context, CornerRadius, Frame, Id, Image, ImageSource,
    Layout, Margin, Order, Pos2, ProgressBar, RichText, Sense, Stroke, StrokeKind, TopBottomPanel,
    Ui, UiBuilder, Vec2, include_image, vec2,
};

use crate::{
    app::{
        constants::{colors, corners},
        i18n::t,
        services::updater::{self, DownloadProgress, UpdateStatus},
        state::{Language, State},
        views::View,
    },
    utils,
};

pub mod base;
pub mod header;
pub mod menu;
pub mod toast;
pub mod updater_modal;
