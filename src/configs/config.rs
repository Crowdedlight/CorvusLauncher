use std::collections::HashMap;
use std::{fmt, fs};
use std::fs::exists;
use std::path::PathBuf;
use std::string::ToString;
use std::sync::{Arc, LazyLock, Mutex, RwLock};
use etcetera::{choose_app_strategy, AppStrategy, AppStrategyArgs, BaseStrategy};
use iced::Task;
use serde::{Deserialize, Serialize};
use crate::messages::Message;

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
    ServerMods
}


#[derive(Deserialize, Serialize, Default, Debug)]
pub struct Config {
    pub a3_root: PathBuf,
    pub a3_server_executable: PathBuf,
    pub folder_modlists: PathBuf,
    pub folder_clientside: PathBuf,
    pub folder_servermods: PathBuf,
    pub server_profiles: Vec<String>
}



impl Config {
    pub fn new() -> Config {

        // check if file exists at default storage location for this OS
        let config_path = &*DEFAULT_CONFIG_FILE_PATH;

        // if not exist, we create it by copying default.toml to it
        if !exists(config_path).expect("failed to check config file path due to permissions or similar issue") {
            log::info!("Could not find configuration file {:?}", config_path);

            // build default empty configs
            let config = Config::default();
            fs::write(config_path, toml::to_string(&config).unwrap()).expect("Failed to make default configs file");

            log::info!("Created new configs file with default values and stored at: {:?}", config_path);
        }

        // Read the configs file and store in struct
        let raw_config = fs::read_to_string(config_path).expect("Could not read configs file");
        let config: Config = toml::from_str(&*raw_config).expect("Could not parse configs file");

        config
    }

    /// check if config is valid
    pub fn is_config_valid(&self) -> bool {
        // check if any paths is empty
        if self.a3_root.to_string_lossy().is_empty() || self.a3_server_executable.to_string_lossy().is_empty() ||
            self.folder_modlists.to_string_lossy().is_empty() || self.folder_clientside.to_string_lossy().is_empty() || self.folder_servermods.to_string_lossy().is_empty() {
            return false;
        }
        // we can do extra validation here, if we want to ensure server binary exists etc...

        true
    }

    /// Update location path in config. Even though it would be more ideal to batch together instead of update config per dialog choice, this should work fine for MVP
    pub fn update_paths(&mut self, location_type: LocationPaths, path: PathBuf) -> anyhow::Result<()> {

        log::debug!("update_paths: type: {:?}, handle: {:?}", location_type, path);

        match location_type {
            LocationPaths::A3Root => {self.update_config(Some(path), None, None, None)}
            LocationPaths::Modlists => {self.update_config(None, Some(path), None, None)}
            LocationPaths::Clientsides => {self.update_config(None, None, Some(path), None)}
            LocationPaths::ServerMods => {self.update_config(None, None, None, Some(path))}
        }
    }

    pub fn update_config(&mut self, a3_root: Option<PathBuf>, folder_modlists: Option<PathBuf>, folder_clientside: Option<PathBuf>, folder_servermods: Option<PathBuf>) -> anyhow::Result<()> {

        // update values
        if let Some(a3_root) = a3_root {
            self.a3_root = a3_root.clone();
            self.a3_server_executable = a3_root.join(A3_SERVER_BINARY_NAME);
            log::debug!("Updated a3_root to: {:?}", self.a3_root);
            log::debug!("Updated a3_server_executable to: {:?}", self.a3_server_executable);
        }
        if let Some(folder_modlists) = folder_modlists {
            self.folder_modlists = folder_modlists;
            log::debug!("Updated folder_modlists to: {:?}", self.folder_modlists);
        }
        if let Some(folder_clientside) = folder_clientside {
            self.folder_clientside = folder_clientside;
            log::debug!("Updated folder_clientside to: {:?}", self.folder_clientside);
        }
        if let Some(folder_servermods) = folder_servermods {
            self.folder_servermods = folder_servermods;
            log::debug!("Updated folder_servermods to: {:?}", self.folder_servermods);
        }

        // update file on disk, by just overwriting it with current configs. (Don't support external file changes without a restart)
        let config = Config::default();
        fs::write(&*DEFAULT_CONFIG_FILE_PATH, toml::to_string(&config).unwrap())?;

        log::info!("Updated configs");
        Ok(())
    }
}

/// function to open file dialog to pick folder. We don't need async as nothing else in app has to run while picking
pub fn open_file_dialog(mut config: Arc<RwLock<Config>>, location: LocationPaths) -> anyhow::Result<()> {

    // if we run into issues with slow saving to disk, either change to save all values at once, or make it async, but that will open new problems...

    // open filedialog
    let selection = rfd::FileDialog::new().pick_folder();

    if let Some(selection) = selection {
        config.write().unwrap().update_paths(location, selection.to_path_buf())
    } else {
        Err(anyhow::anyhow!("Have to pick a folder..."))
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


