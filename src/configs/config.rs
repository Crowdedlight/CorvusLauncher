use crate::messages::Message;
use etcetera::{AppStrategy, AppStrategyArgs, BaseStrategy, choose_app_strategy};
use iced::Task;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::default::Default;
use std::fs::exists;
use std::path::PathBuf;
use std::string::ToString;
use std::sync::{Arc, LazyLock, Mutex, RwLock};
use std::{fmt, fs};

/// Arma server binary name depending on platform
#[cfg(target_os = "linux")]
static A3_SERVER_BINARY_NAME: &str = "arma3server_x64";
#[cfg(target_os = "windows")]
static A3_SERVER_BINARY_NAME: &str = "arma3server_x64.exe";

#[derive(Clone, Debug)]
pub enum LocationPaths {
    A3Root,
    Modlists,
    Clientsides,
    ServerMods,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub a3_root: PathBuf,
    pub a3_server_executable: PathBuf,
    pub folder_modlists: PathBuf,
    pub folder_clientside: PathBuf,
    pub folder_servermods: PathBuf,
    pub server_profiles: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            a3_root: Default::default(),
            folder_servermods: Default::default(),
            folder_clientside: Default::default(),
            folder_modlists: Default::default(),
            a3_server_executable: Default::default(),
            server_profiles: vec!["ServerNormal".to_string()],
        }
    }
}

impl Config {
    pub fn new() -> Config {
        // check if file exists at default storage location for this OS
        let config_path = &*DEFAULT_CONFIG_FILE_PATH;

        // if not exist, we create it by copying default.toml to it
        if !exists(config_path)
            .expect("failed to check config file path due to permissions or similar issue")
        {
            log::info!("Could not find configuration file {:?}", config_path);

            // build default empty configs
            let config = Config::default();
            fs::write(config_path, toml::to_string(&config).unwrap())
                .expect("Failed to make default configs file");

            log::info!(
                "Created new configs file with default values and stored at: {:?}",
                config_path
            );
        }

        // Read the configs file and store in struct
        let raw_config = fs::read_to_string(config_path).expect("Could not read configs file");
        let config: Config = toml::from_str(&*raw_config).expect("Could not parse configs file");

        config
    }

    /// check if config is valid
    pub fn is_config_valid(&self) -> bool {
        // check if any paths is empty
        if self.a3_root.to_string_lossy().is_empty()
            || self.a3_server_executable.to_string_lossy().is_empty()
            || self.folder_modlists.to_string_lossy().is_empty()
            || self.folder_clientside.to_string_lossy().is_empty()
            || self.folder_servermods.to_string_lossy().is_empty()
        {
            return false;
        }
        // we can do extra validation here, if we want to ensure server binary exists etc...

        true
    }

    pub fn update_config(
        &mut self,
        a3_root: PathBuf,
        folder_modlists: PathBuf,
        folder_clientside: PathBuf,
        folder_servermods: PathBuf,
    ) -> anyhow::Result<()> {
        // update values
        self.a3_root = a3_root.clone();
        self.a3_server_executable = a3_root.join(A3_SERVER_BINARY_NAME);
        log::debug!("Updated a3_root to: {:?}", self.a3_root);
        log::debug!(
            "Updated a3_server_executable to: {:?}",
            self.a3_server_executable
        );

        self.folder_modlists = folder_modlists;
        log::debug!("Updated folder_modlists to: {:?}", self.folder_modlists);

        self.folder_clientside = folder_clientside;
        log::debug!("Updated folder_clientside to: {:?}", self.folder_clientside);

        self.folder_servermods = folder_servermods;
        log::debug!("Updated folder_servermods to: {:?}", self.folder_servermods);

        // update file on disk, by just overwriting it with current configs. (Don't support external file changes without a restart)
        let config = Config::default();
        fs::write(
            &*DEFAULT_CONFIG_FILE_PATH,
            toml::to_string(&config).unwrap(),
        )?;

        log::info!("Updated configs");
        Ok(())
    }
}

/// Represents the default location of the configs file
pub static DEFAULT_CONFIG_FILE_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    etcetera::choose_base_strategy().map_or_else(
        |err| {
            log::warn!("Could not determine the configs directory: {err}");
            PathBuf::from("corvuslauncher.toml")
        },
        |strategy| strategy.config_dir().join("corvuslauncher.toml"),
    )
});

/// Represents the default location of the configs file
pub static DEFAULT_LOG_FILE_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    etcetera::choose_base_strategy().map_or_else(
        |err| {
            log::warn!("Could not determine the configs directory: {err}");
            PathBuf::from("corvuslauncher.log")
        },
        |strategy| strategy.cache_dir().join("corvuslauncher.log"),
    )
});
