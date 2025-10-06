pub mod cli;
pub mod config;

pub use cli::Cli;
use std::path::PathBuf;
pub use config::DEFAULT_LOG_FILE_PATH;
pub use config::Config;

/// The default configuration, to be merged with the user's configs
///
/// When modifying any of the configs options, this will also need to be updated
pub const DEFAULT_YAML_CONFIG_STR: &str = include_str!("../../assets/default.toml");

// TODO implement configs with sync to file