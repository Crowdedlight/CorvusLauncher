pub mod cli;

pub use cli::Cli;
use std::path::PathBuf;
pub use cli::DEFAULT_LOG_FILE_PATH;

/// The default configuration, to be merged with the user's config
///
/// When modifying any of the config options, this will also need to be updated
pub const DEFAULT_YAML_CONFIG_STR: &str = include_str!("../../default.yaml");

// TODO implement config with sync to file