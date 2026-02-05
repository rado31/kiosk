use std::{env, fs::File};

use simplelog::{ColorChoice, ConfigBuilder, LevelFilter, TermLogger, TerminalMode, WriteLogger};

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        log::debug!($($arg)*)
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        log::info!($($arg)*)
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        log::error!($($arg)*)
    };
}

/// Initialize logger based on DEBUG environment variable:
/// - DEBUG=1: debug + info + error to terminal
/// - Default: info + error to file
pub fn init() {
    let is_debug = env::var("DEBUG").map(|v| v == "1").unwrap_or(false);
    let config = ConfigBuilder::new().add_filter_allow_str("kiosk").build();

    if is_debug {
        TermLogger::init(
            LevelFilter::Debug,
            config,
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

    WriteLogger::init(LevelFilter::Info, config, file).unwrap();
}
