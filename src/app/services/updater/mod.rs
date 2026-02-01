#![allow(dead_code)]

use std::{
    env, fs,
    io::Read,
    path::{Path, PathBuf},
    process::Command,
    sync::mpsc,
    thread,
};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

use ed25519_dalek::{PUBLIC_KEY_LENGTH, Signature, Verifier, VerifyingKey};
use egui::Context;
use reqwest::blocking::Client;
use semver::Version;

use crate::{
    error,
    errors::{AppError, Result},
    utils,
};

mod types;

pub use types::{DownloadProgress, NewUpdate, UpdateInfo, UpdateMessage, UpdateStatus};

const UPDATE_URL: &str = "http://localhost:8080/info";
const PUBLIC_KEY: &[u8; PUBLIC_KEY_LENGTH] = include_bytes!("../../../../keys/public.key");

/// Fetches for update in a background thread.
/// Returns a receiver to get update messages.
pub fn start_check(ctx: &Context) -> mpsc::Receiver<UpdateMessage> {
    let (tx, rx) = mpsc::channel();
    let ctx = ctx.clone();

    thread::spawn(move || {
        let opt_info = match check(UPDATE_URL) {
            Ok(v) => v,
            Err(e) => {
                error!("Check new update. {e}");
                tx.send(UpdateMessage::Done).ok();
                return;
            }
        };

        if let Some(info) = opt_info {
            let tx_progress = tx.clone();
            let ctx_progress = ctx.clone();
            let version = info.version.clone();

            let res = download(&info, |downloaded, total| {
                tx_progress
                    .send(UpdateMessage::Progress(DownloadProgress {
                        downloaded,
                        total,
                        version: version.clone(),
                    }))
                    .ok();

                ctx_progress.request_repaint();
            });

            match res {
                Ok(path) => {
                    tx.send(UpdateMessage::Downloaded(path)).ok();
                }
                Err(e) => {
                    error!("Download update. {e}");
                    tx.send(UpdateMessage::Done).ok();
                }
            };

            ctx.request_repaint();
        } else {
            tx.send(UpdateMessage::Done).ok();
            ctx.request_repaint();
        };
    });

    rx
}

/// Check if an update is available.
/// Returns `Ok(Some(UpdateInfo))` if a newer version exists, `Ok(None)` if current is latest.
fn check(url: &str) -> Result<Option<UpdateInfo>> {
    let client = Client::new();
    let response = client.get(url).send()?;
    let info: UpdateInfo = response.json()?;
    let current = Version::parse(env!("CARGO_PKG_VERSION"))?;
    let remote = Version::parse(&info.version)?;

    if remote > current {
        return Ok(Some(info));
    }

    Ok(None)
}

/// Download the update and verify its signature.
/// Returns the path to the verified binary on success.
/// Calls the progress callback with (downloaded_bytes, total_bytes).
fn download<F>(info: &UpdateInfo, mut on_progress: F) -> Result<PathBuf>
where
    F: FnMut(u64, u64),
{
    let client = Client::new();
    let response = client.get(&info.url).send()?;
    let total = response.content_length().unwrap_or(0);
    let mut downloaded: u64 = 0;
    let mut bytes = Vec::new();
    let mut reader = response;
    let mut buffer = [0u8; 8192];

    loop {
        let n = reader.read(&mut buffer)?;

        if n == 0 {
            break;
        }

        bytes.extend_from_slice(&buffer[..n]);
        downloaded += n as u64;
        on_progress(downloaded, total);

        // TODO: remove in production,
        // becuase it is just for testing, to see a modal window with progress bar
        utils::sleep(10);
    }

    let signature_bytes = base64_decode(&info.signature).ok_or_else(|| AppError::Base64Decode)?;
    let signature = Signature::from_slice(&signature_bytes)?;
    let public_key = VerifyingKey::from_bytes(PUBLIC_KEY)?;

    public_key.verify(&bytes, &signature)?;

    let temp_name = if cfg!(windows) {
        "kiosk_update.exe"
    } else {
        "kiosk_update"
    };

    let temp_path = env::temp_dir().join(temp_name);

    fs::write(&temp_path, &bytes)?;

    Ok(temp_path)
}

/// Clean up old binary from previous update (call on app startup).
pub fn cleanup_old_binary() {
    if let Ok(current_exe) = env::current_exe() {
        let old_exe = current_exe.with_extension("old");

        fs::remove_file(old_exe).ok();

        #[cfg(windows)]
        {
            let old_exe_win = PathBuf::from(format!("{}.old", current_exe.display()));

            fs::remove_file(old_exe_win).ok();
        }
    }
}

/// Install the new binary and restart the application.
/// This function does not return on success.
pub fn install_and_restart(new_binary: &Path) -> Result<()> {
    let current_exe = env::current_exe()?;

    #[cfg(windows)]
    {
        // Windows: can't replace running executable, rename it first
        let old_exe = PathBuf::from(format!("{}.old", current_exe.display()));

        fs::remove_file(&old_exe).ok();
        fs::rename(&current_exe, &old_exe)?;
        fs::copy(new_binary, &current_exe)?;
    }

    #[cfg(unix)]
    {
        // Unix: remove the running executable first (it stays in memory until process exits)
        fs::remove_file(&current_exe)?;
        fs::copy(new_binary, &current_exe)?;

        let mut perms = fs::metadata(&current_exe)?.permissions();

        perms.set_mode(0o755);

        fs::set_permissions(&current_exe, perms)?;
    }

    fs::remove_file(new_binary).ok();

    Command::new(&current_exe).spawn()?;
    std::process::exit(0);
}

/// Simple base64 decoder (standard alphabet, no padding required)
fn base64_decode(input: &str) -> Option<Vec<u8>> {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let input = input.trim_end_matches('=');
    let mut output = Vec::with_capacity(input.len() * 3 / 4);
    let mut buffer: u32 = 0;
    let mut bits_collected: u8 = 0;

    for c in input.bytes() {
        let value = ALPHABET.iter().position(|&x| x == c)? as u32;

        buffer = (buffer << 6) | value;
        bits_collected += 6;

        if bits_collected >= 8 {
            bits_collected -= 8;
            output.push((buffer >> bits_collected) as u8);
            buffer &= (1 << bits_collected) - 1;
        }
    }

    Some(output)
}
