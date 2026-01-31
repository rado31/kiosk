use std::{path::PathBuf, sync::mpsc};

use serde::Deserialize;

/// Information about an available update from the server.
#[derive(Debug, Deserialize)]
pub struct UpdateInfo {
    pub version: String,
    pub url: String,
    pub signature: String,
}

/// Progress of an ongoing download.
#[derive(Default, Clone)]
pub struct DownloadProgress {
    pub downloaded: u64,
    pub total: u64,
    pub version: String,
}

/// Current status of the update process.
#[derive(Default, Clone)]
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

/// Messages sent from the update background thread.
pub enum UpdateMessage {
    Progress(DownloadProgress),
    Downloaded(PathBuf),
    Done,
}

/// Update statement
#[derive(Default)]
pub struct NewUpdate {
    pub status: UpdateStatus,
    pub receiver: Option<mpsc::Receiver<UpdateMessage>>,
}
