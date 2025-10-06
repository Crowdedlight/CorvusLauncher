use crate::configs::config::DEFAULT_LOG_FILE_PATH;
use crate::configs::config::DEFAULT_CONFIG_FILE_PATH;
use std::path::PathBuf;
use std::sync::LazyLock;
use clap::ValueHint;
use clap::builder::styling::{AnsiColor, Effects};
use clap::Parser;
use etcetera::BaseStrategy;

// styling for errors
const STYLES: clap::builder::Styles = clap::builder::Styles::styled()
    .header(AnsiColor::BrightGreen.on_default().effects(Effects::BOLD))
    .usage(AnsiColor::BrightGreen.on_default().effects(Effects::BOLD))
    .literal(AnsiColor::BrightCyan.on_default().effects(Effects::BOLD))
    .placeholder(AnsiColor::BrightCyan.on_default())
    .error(AnsiColor::BrightRed.on_default().effects(Effects::BOLD))
    .valid(AnsiColor::BrightCyan.on_default().effects(Effects::BOLD))
    .invalid(AnsiColor::BrightYellow.on_default().effects(Effects::BOLD));


#[derive(Parser, Debug)]
#[command(version, styles = STYLES, long_about = None)]
#[expect(clippy::struct_excessive_bools, reason = "normal for CLIs")]
pub struct Cli {
    /// Use the provided configs file
    #[arg(
        help_heading = "Config",
        short = 'C',
        long,
        value_name = "FILE.KDL",
        default_value_t = DEFAULT_CONFIG_FILE_PATH.to_string_lossy().to_string(),
        value_hint = ValueHint::FilePath
    )]
    pub config_file: String,

    //
    // --- Debug ---
    //
    // Requires to be compiled with `debug` for them to show up in the CLI help
    //
    /// Choose a miniumum level at which to log
    #[arg(
        help_heading = "Debug",
        long,
        value_name = "LEVEL",
        default_value = "error",
        long_help = "Choose a minimum level at which to log. [error, warn, info, debug, trace, off]",
        hide = !cfg!(feature = "debug")
    )]
    pub log_level: log::LevelFilter,

    /// Log to standard error instead of file
    #[arg(
        help_heading = "Debug",
        long,
        hide = !cfg!(feature = "debug")
    )]
    pub log_stderr: bool,

    /// Path to the log file
    #[arg(
        help_heading = "Debug",
        long,
        value_name = "FILE",
        default_value_t = DEFAULT_LOG_FILE_PATH.to_string_lossy().to_string(),
        value_hint = ValueHint::FilePath,
        hide = !cfg!(feature = "debug")
    )]
    pub log_file: String,

    /// Filter for specific Rust module or crate, instead of showing logs from all crates
    #[arg(
        help_heading = "Debug",
        long,
        value_name = "FILTER",
        value_hint = ValueHint::Other,
        hide = !cfg!(feature = "debug")
    )]
    pub log_filter: Option<String>,

    /// Launch in debug mode (F12)
    #[arg(
        help_heading = "Debug",
        long,
        hide = !cfg!(feature = "debug")
    )]
    pub debug: bool,
}

