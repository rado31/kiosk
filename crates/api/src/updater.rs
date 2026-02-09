use std::{
    env, fs,
    io::Read,
    path::{Path, PathBuf},
    process::Command,
};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

use ed25519_dalek::{PUBLIC_KEY_LENGTH, Signature, Verifier, VerifyingKey};
use core::{AppError, Result, base64};
use serde::Deserialize;

const PUBLIC_KEY: &[u8; PUBLIC_KEY_LENGTH] = include_bytes!("../../../keys/public.key");

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

/// Check if an update is available.
/// Returns `Ok(Some(UpdateInfo))` if a newer version exists, `Ok(None)` if current is latest.
pub fn check() -> Result<Option<UpdateInfo>> {
    let agent = ureq::Agent::new_with_defaults();
    let mut body = agent
        .get(core::config::UPDATE_URL)
        .call()?
        .into_body();

    let info: UpdateInfo = body.read_json()?;
    let current = semver::Version::parse(env!("CARGO_PKG_VERSION"))?;
    let remote = semver::Version::parse(&info.version)?;

    if remote > current {
        return Ok(Some(info));
    }

    Ok(None)
}

/// Download the update and verify its signature.
/// Returns the path to the verified binary on success.
/// Calls the progress callback with (downloaded_bytes, total_bytes).
pub fn download<F>(info: &UpdateInfo, mut on_progress: F) -> Result<PathBuf>
where
    F: FnMut(u64, u64),
{
    let agent = ureq::Agent::new_with_defaults();
    let response = agent.get(&info.url).call()?;

    let total = response
        .headers()
        .get("content-length")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(0);

    let mut reader = response.into_body().into_reader();
    let mut downloaded: u64 = 0;
    let mut bytes = Vec::new();
    let mut buffer = [0u8; 8192];

    loop {
        let n = reader.read(&mut buffer)?;

        if n == 0 {
            break;
        }

        bytes.extend_from_slice(&buffer[..n]);
        downloaded += n as u64;
        on_progress(downloaded, total);
    }

    let signature_bytes = base64::decode(&info.signature).ok_or(AppError::Base64Decode)?;
    let signature =
        Signature::from_slice(&signature_bytes).map_err(|_| AppError::InvalidSignature)?;
    let public_key =
        VerifyingKey::from_bytes(PUBLIC_KEY).map_err(|_| AppError::InvalidSignature)?;

    public_key
        .verify(&bytes, &signature)
        .map_err(|_| AppError::InvalidSignature)?;

    let temp_name = if cfg!(windows) {
        "kiosk_update.exe"
    } else {
        "kiosk_update"
    };

    let temp_path = env::temp_dir().join(temp_name);

    fs::write(&temp_path, &bytes)?;

    Ok(temp_path)
}

/// Install the new binary and restart the application.
/// This function does not return on success.
pub fn install_and_restart(new_binary: &Path) -> Result<()> {
    let current_exe = env::current_exe()?;

    #[cfg(windows)]
    {
        let old_exe = PathBuf::from(format!("{}.old", current_exe.display()));

        fs::remove_file(&old_exe).ok();
        fs::rename(&current_exe, &old_exe)?;
        fs::copy(new_binary, &current_exe)?;
    }

    #[cfg(unix)]
    {
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
