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
pub fn download_update(info: &UpdateInfo) -> Result<PathBuf, UpdateError> {
    let client = Client::new();
    let response = client.get(&info.url).send()?;
    let bytes = response.bytes()?;

    // Decode base64 signature
    let signature_bytes = base64_decode(&info.signature).ok_or(UpdateError::Base64Decode)?;

    if signature_bytes.len() != SIGNATURE_LENGTH {
        return Err(UpdateError::InvalidSignature);
    }

    let signature =
        Signature::from_slice(&signature_bytes).map_err(|_| UpdateError::InvalidSignature)?;

    // Verify signature
    let public_key =
        VerifyingKey::from_bytes(PUBLIC_KEY).map_err(|_| UpdateError::InvalidPublicKey)?;

    public_key
        .verify(&bytes, &signature)
        .map_err(|_| UpdateError::InvalidSignature)?;

    // Save to temp file (with .exe extension on Windows)
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
    let current_exe = env::current_exe()?;

    #[cfg(windows)]
    {
        // Windows: can't replace running executable, rename it first
        let old_exe = PathBuf::from(format!("{}.old", current_exe.display()));
        fs::remove_file(&old_exe).ok(); // Remove any existing .old file
        fs::rename(&current_exe, &old_exe)?;
        fs::copy(new_binary, &current_exe)?;
    }

    #[cfg(unix)]
    {
        // Unix: can replace running executable directly
        fs::copy(new_binary, &current_exe)?;

        // Set executable permissions
        let mut perms = fs::metadata(&current_exe)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&current_exe, perms)?;
    }

    // Clean up temp file
    fs::remove_file(new_binary).ok();

    // Spawn new process and exit
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
