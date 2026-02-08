use std::{path::PathBuf, sync::mpsc};

#[derive(Default)]
pub struct DownloadProgress {
    pub downloaded: u64,
    pub total: u64,
    pub version: String,
}

#[derive(Default)]
#[allow(dead_code)]
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

#[allow(dead_code)]
pub enum UpdateMessage {
    Progress(DownloadProgress),
    Downloaded(PathBuf),
    Done,
}

#[derive(Default)]
pub struct State {
    pub status: UpdateStatus,
    pub receiver: Option<mpsc::Receiver<UpdateMessage>>,
}
