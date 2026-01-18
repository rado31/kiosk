use std::{sync::mpsc, thread};

use egui::{
    Align, Area, Button, Context, Frame, Id, Image, Layout, Margin, Order, RichText, Shadow,
    Stroke, Ui, include_image, pos2, vec2,
};

use crate::{
    app::{Language, State, UpdateStatus, constants},
    updater,
};

pub mod header;
pub mod menu;
