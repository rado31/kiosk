use std::{env, fs::File};

use simplelog::{ColorChoice, Config, LevelFilter, TermLogger, TerminalMode, WriteLogger};

/// Initialize logger based on DEBUG environment variable:
/// - DEBUG=true: error + info to terminal
/// - Default (no DEBUG or DEBUG=false): error to file
pub fn init() {
    let is_debug = env::var("DEBUG").map(|v| v == "1").unwrap_or(false);

    if is_debug {
        TermLogger::init(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        )
        .unwrap();

        return;
    }

    let log_path = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join("error.log");

    let file = File::create(&log_path).unwrap();

    WriteLogger::init(LevelFilter::Error, Config::default(), file).unwrap();
}
