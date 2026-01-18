use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

use ed25519_dalek::{PUBLIC_KEY_LENGTH, SIGNATURE_LENGTH, Signature, Verifier, VerifyingKey};
use reqwest::blocking::Client;
use semver::Version;
use serde::Deserialize;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

const PUBLIC_KEY: &[u8; PUBLIC_KEY_LENGTH] = include_bytes!("../../keys/public.key");

#[derive(Debug, Deserialize)]
pub struct UpdateInfo {
    pub version: String,
    pub url: String,
    pub signature: String,
}

#[derive(Debug)]
pub enum UpdateError {
    Network(reqwest::Error),
    Json(serde_json::Error),
    Version(semver::Error),
    Io(std::io::Error),
    InvalidSignature,
    InvalidPublicKey,
    Base64Decode,
}

impl From<reqwest::Error> for UpdateError {
    fn from(e: reqwest::Error) -> Self {
        UpdateError::Network(e)
    }
}

impl From<serde_json::Error> for UpdateError {
    fn from(e: serde_json::Error) -> Self {
        UpdateError::Json(e)
    }
}

impl From<semver::Error> for UpdateError {
    fn from(e: semver::Error) -> Self {
        UpdateError::Version(e)
    }
}

impl From<std::io::Error> for UpdateError {
    fn from(e: std::io::Error) -> Self {
        UpdateError::Io(e)
    }
}

/// Check if an update is available.
/// Returns `Ok(Some(UpdateInfo))` if a newer version exists, `Ok(None)` if current is latest.
pub fn check_for_update(url: &str) -> Result<Option<UpdateInfo>, UpdateError> {
    eprintln!("[updater] Checking for updates at: {}", url);

    let client = Client::new();
    let response = client.get(url).send().map_err(|e| {
        eprintln!("[updater] Network error: {}", e);
        UpdateError::Network(e)
    })?;

    eprintln!("[updater] Response status: {}", response.status());

    let info: UpdateInfo = response.json().map_err(|e| {
        eprintln!("[updater] JSON parse error: {}", e);
        UpdateError::Network(e)
    })?;

    eprintln!("[updater] Remote version: {}", info.version);

    let current = Version::parse(env!("CARGO_PKG_VERSION")).map_err(|e| {
        eprintln!("[updater] Current version parse error: {}", e);
        UpdateError::Version(e)
    })?;

    let remote = Version::parse(&info.version).map_err(|e| {
        eprintln!("[updater] Remote version parse error: {}", e);
        UpdateError::Version(e)
    })?;

    eprintln!("[updater] Current: {} | Remote: {}", current, remote);

    if remote > current {
        eprintln!("[updater] Update available!");
        return Ok(Some(info));
    }

    eprintln!("[updater] Already up to date");
    Ok(None)
}

/// Download the update and verify its signature.
/// Returns the path to the verified binary on success.
pub fn download_update(info: &UpdateInfo) -> Result<PathBuf, UpdateError> {
    eprintln!("[updater] Downloading update from: {}", info.url);

    let client = Client::new();
    let response = client.get(&info.url).send().map_err(|e| {
        eprintln!("[updater] Download error: {}", e);
        UpdateError::Network(e)
    })?;

    eprintln!("[updater] Download status: {}", response.status());

    let bytes = response.bytes().map_err(|e| {
        eprintln!("[updater] Failed to read response bytes: {}", e);
        UpdateError::Network(e)
    })?;

    eprintln!("[updater] Downloaded {} bytes", bytes.len());

    // Decode base64 signature
    let signature_bytes = base64_decode(&info.signature).ok_or_else(|| {
        eprintln!("[updater] Failed to decode base64 signature");
        UpdateError::Base64Decode
    })?;

    eprintln!("[updater] Signature length: {} bytes", signature_bytes.len());

    if signature_bytes.len() != SIGNATURE_LENGTH {
        eprintln!(
            "[updater] Invalid signature length: expected {}, got {}",
            SIGNATURE_LENGTH,
            signature_bytes.len()
        );
        return Err(UpdateError::InvalidSignature);
    }

    let signature = Signature::from_slice(&signature_bytes).map_err(|e| {
        eprintln!("[updater] Invalid signature format: {}", e);
        UpdateError::InvalidSignature
    })?;

    // Verify signature
    let public_key = VerifyingKey::from_bytes(PUBLIC_KEY).map_err(|e| {
        eprintln!("[updater] Invalid public key: {}", e);
        UpdateError::InvalidPublicKey
    })?;

    eprintln!("[updater] Verifying signature...");

    public_key.verify(&bytes, &signature).map_err(|e| {
        eprintln!("[updater] Signature verification failed: {}", e);
        UpdateError::InvalidSignature
    })?;

    eprintln!("[updater] Signature verified successfully");

    // Save to temp file (with .exe extension on Windows)
    let temp_name = if cfg!(windows) {
        "kiosk_update.exe"
    } else {
        "kiosk_update"
    };

    let temp_path = env::temp_dir().join(temp_name);

    fs::write(&temp_path, &bytes).map_err(|e| {
        eprintln!("[updater] Failed to write temp file: {}", e);
        UpdateError::Io(e)
    })?;

    eprintln!("[updater] Saved to: {}", temp_path.display());

    Ok(temp_path)
}

/// Clean up old binary from previous update (call on app startup).
pub fn cleanup_old_binary() {
    if let Ok(current_exe) = env::current_exe() {
        let old_exe = current_exe.with_extension("old");
        fs::remove_file(old_exe).ok();

        // Windows may also have .exe.old
        #[cfg(windows)]
        {
            let old_exe_win = PathBuf::from(format!("{}.old", current_exe.display()));
            fs::remove_file(old_exe_win).ok();
        }
    }
}

/// Install the new binary and restart the application.
/// This function does not return on success.
pub fn install_and_restart(new_binary: &Path) -> Result<(), UpdateError> {
    eprintln!("[updater] Installing update from: {}", new_binary.display());

    let current_exe = env::current_exe().map_err(|e| {
        eprintln!("[updater] Failed to get current exe path: {}", e);
        UpdateError::Io(e)
    })?;

    eprintln!("[updater] Current executable: {}", current_exe.display());

    #[cfg(windows)]
    {
        // Windows: can't replace running executable, rename it first
        let old_exe = PathBuf::from(format!("{}.old", current_exe.display()));
        eprintln!("[updater] Renaming current exe to: {}", old_exe.display());
        fs::remove_file(&old_exe).ok();
        fs::rename(&current_exe, &old_exe).map_err(|e| {
            eprintln!("[updater] Failed to rename current exe: {}", e);
            UpdateError::Io(e)
        })?;
        fs::copy(new_binary, &current_exe).map_err(|e| {
            eprintln!("[updater] Failed to copy new binary: {}", e);
            UpdateError::Io(e)
        })?;
    }

    #[cfg(unix)]
    {
        // Unix: remove the running executable first (it stays in memory until process exits)
        fs::remove_file(&current_exe).map_err(|e| {
            eprintln!("[updater] Failed to remove current binary: {}", e);
            UpdateError::Io(e)
        })?;
        eprintln!("[updater] Removed current executable");

        fs::copy(new_binary, &current_exe).map_err(|e| {
            eprintln!("[updater] Failed to copy new binary: {}", e);
            UpdateError::Io(e)
        })?;

        // Set executable permissions
        let mut perms = fs::metadata(&current_exe)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&current_exe, perms).map_err(|e| {
            eprintln!("[updater] Failed to set permissions: {}", e);
            UpdateError::Io(e)
        })?;
        eprintln!("[updater] Set executable permissions");
    }

    // Clean up temp file
    fs::remove_file(new_binary).ok();
    eprintln!("[updater] Cleaned up temp file");

    eprintln!("[updater] Restarting application...");
    Command::new(&current_exe).spawn().map_err(|e| {
        eprintln!("[updater] Failed to spawn new process: {}", e);
        UpdateError::Io(e)
    })?;

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
