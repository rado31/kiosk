use std::{path::PathBuf, sync::mpsc};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct UpdateInfo {
    pub version: String,
    pub url: String,
    pub signature: String,
}

#[derive(Default)]
pub struct DownloadProgress {
    pub downloaded: u64,
    pub total: u64,
    pub version: String,
}

#[derive(Default)]
pub enum UpdateStatus {
    #[default]
    Idle,
    Checking,
    Downloading(DownloadProgress),
}

impl UpdateStatus {
    pub fn downloading(&self) -> Option<&DownloadProgress> {
        match self {
            Self::Downloading(progress) => Some(progress),
            _ => None,
        }
    }
}

pub enum UpdateMessage {
    Progress(DownloadProgress),
    Downloaded(PathBuf),
    Done,
}

#[derive(Default)]
pub struct NewUpdate {
    pub status: UpdateStatus,
    pub receiver: Option<mpsc::Receiver<UpdateMessage>>,
}
