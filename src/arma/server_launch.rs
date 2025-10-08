


// TODO, do like the powershell script
//  Check if A3 process is running, then show error and ask to close it first
//  empty keys folder
//  Find and copy keys from all selected mods into keys folder. (Require function to search for the .bikey file in subfolders)
//  Repeat for Clientsides
//  Make .txt file with "-mods=..." for all the mods
//  Build Commandline launch string, server profile, mods, server mods
//  Launch process with launch string as child process, disown process, so we can close this launcher without killing server or HC instance

use std::path::PathBuf;
use anyhow::Result;

pub fn find_bikey(path: PathBuf) -> Result<Vec<PathBuf>> {

    // search all subfiles for a .bikey
    let keys = std::fs::read_dir(path.clone())?
        // Filter out all those directory entries which couldn't be read
        .filter_map(|res| res.ok())
        // Map the directory entries to paths
        .map(|dir_entry| dir_entry.path())
        // Filter out all paths with extensions other than `csv`
        .filter_map(|path| {
            if path.extension().is_some_and(|ext| ext == "bikey") {
                Some(path)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    // Logging
    log::debug!(
        "Found Following bikeys in mod, {:?}: {:?}",
        path.file_name(), keys
    );

    match keys.is_empty() {
        true => Err(anyhow::Error::msg(format!("No Bikey found in mod: {:?}", path.file_name()))),
        false => Ok(keys),
    }
}

pub fn launch_hc() -> Result<()> {
    // TODO server password for HCs? Or should we just steal that from the server config file. We know the A3root ;-) ?



    Ok(())
}
/// Find the .cfg file in the A3Root to parse it and get the password
pub fn find_config(a3root: PathBuf) -> Result<PathBuf> {

    let keys = std::fs::read_dir(a3root)?
        // Filter out all those directory entries which couldn't be read
        .filter_map(|res| res.ok())
        // Filter out all paths with extensions other than `csv`
        .filter_map(|f| {
            if f.path().extension().is_some_and(|ext| ext == "cfg") {
                Some(f)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    // should only return one, as the a3root should only have a single .cfg file
    match keys.first() {
        Some(keys) => {Ok(keys.path())},
        None => {Err(anyhow::Error::msg("A3 Server Config not found in A3Root"))}
    }
}

/// Given the path to the A3 server config file, extract the password for use in HCs launch parameters
pub fn get_server_password_from_config(a3_config: PathBuf) -> Result<String> {
    // find the password which is given in the file as: password = "thepassword";
    let config_string = std::fs::read_to_string(a3_config)?;

    // TODO regex to find and extract the password string from: password = "thepassword";
    config_string

    Ok("".to_string())
}