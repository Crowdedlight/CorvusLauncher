


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