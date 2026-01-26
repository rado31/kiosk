use egui::{
    Align, Align2, Area, Button, Color32, Context, CornerRadius, Frame, Id, Image, ImageSource,
    Layout, Margin, Order, Pos2, ProgressBar, RichText, Sense, Stroke, StrokeKind, TopBottomPanel,
    Ui, UiBuilder, Vec2, include_image, vec2,
};

use crate::{
    app::{
        constants::colors,
        i18n::t,
        routes::Route,
        services::updater::{self, DownloadProgress, UpdateStatus},
        state::{Language, State},
    },
    utils,
};

pub mod base;
pub mod header;
pub mod menu;
pub mod updater_modal;
