use crate::arma::preset;
use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

static LOADED_MODS_FILE: &str = "corvuslauncher_loaded_mods.txt";

pub fn find_bikey(path: &Path) -> Result<Vec<PathBuf>> {
    // search all subfiles for a .bikey
    let keys = std::fs::read_dir(path)?
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
        path.file_name(),
        keys
    );

    match keys.is_empty() {
        true => Err(anyhow::Error::msg(format!(
            "No Bikey found in mod: {:?}",
            path.file_name()
        ))),
        false => Ok(keys),
    }
}

/// function that given all mods, create the -par file for server to load.
/// Input path for mods should be the ones loaded in preset, so relative to A3root
pub fn build_mods_launch_file(mods: Vec<PathBuf>, output_file: &PathBuf) -> Result<()> {
    // we got a vector of mods, build the string and output to file
    let mut mod_string = String::with_capacity(mods.len() * 15);
    mod_string.push_str("-mod=");

    for path in mods {
        mod_string.push_str(&path.to_string_lossy());
        mod_string.push(';');
    }

    // save file
    fs::write(&output_file, mod_string)?;

    log::info!("Build server modlist and saved to parameter file");
    log::debug!(
        "Build server modlist successfully and saved to: {:?}",
        output_file
    );

    Ok(())
}

fn remove_dir_contents_but_a3key(path: &PathBuf) -> anyhow::Result<()> {
    for entry in fs::read_dir(path)? {
        let file = entry?;
        // do not remove a3.bikey, as that should always be there
        if !file.file_name().eq("a3") {
            fs::remove_file(file.path())?;
        }
    }
    Ok(())
}

/// launch server with given parameters
pub fn launch_server(
    a3root: PathBuf,
    a3_executable: PathBuf,
    port: u64,
    server_profile: &str,
    modlist: preset::Preset,
    clientsides: preset::Preset,
    server_mods: preset::Preset,
) -> Result<()> {
    let keys_folder = a3root.clone().join("keys");
    let par_modlist = a3root.clone().join(LOADED_MODS_FILE);

    // clean existing keys folder of all keys, besides a3.bikey
    remove_dir_contents_but_a3key(&keys_folder)?;

    // make vecs of paths to bikeys
    let mut bikeys: Vec<PathBuf> = Vec::new();
    let mut missing_keys: Vec<String> = Vec::new();

    // find list of bikeys for all mods, clientsides, modlist and server-mods. Make into one big vec
    let server_and_clientsides = [modlist.mods.clone(), clientsides.mods].concat();

    for modpath in server_and_clientsides.iter() {
        // try and find bikeys
        match find_bikey(modpath) {
            Ok(mut path) => {
                bikeys.append(&mut path);
            }
            Err(e) => missing_keys.push(e.to_string()),
        }
    }

    // if err vec is not empty, we halt the launch and displays errors to users instead of missing bikeys
    if !missing_keys.is_empty() {
        for e in &missing_keys {
            log::error!("{}", e);
        }
        return Err(anyhow::Error::msg(missing_keys.join("\n")));
    }

    // no keys are missing, we can continue by copying keys to the a3root/keys folder
    for p in bikeys.iter() {
        fs::copy(p, &keys_folder)?;
    }

    // build parameter file for server mods
    build_mods_launch_file(modlist.mods, &par_modlist)?;

    // build string for server mods
    let server_mod_string_vec: Vec<String> = server_mods
        .mods
        .iter()
        .map(|entry| String::from(entry.to_string_lossy()))
        .collect();

    // keys sorted, mods collected, time to build the launch string
    let server_launch_string = format!(
        "{server_exe:?} -port={port} -hugepages -maxMem=30000 -maxFileCacheSize=8192 -enableHT -bandwidthAlg=2 -limitFPS=1000 -loadMissionToMemory \
        -profiles={profile} -name=server -config={server_config} -cfg={network_cfg} -world=empty -serverMod=\"{server_mods}\" -par={modfile}",
        server_exe = a3_executable.to_string_lossy(),
        port = port,
        profile = a3root.clone().join(server_profile).to_string_lossy(),
        server_config = find_config(&a3root)?.to_string_lossy(),
        network_cfg = a3root
            .clone()
            .join(server_profile)
            .join("Users/server/Arma3.cfg")
            .to_string_lossy(), //$a3root\$serverprofile\Users\server\Arma3.cfg
        server_mods = server_mod_string_vec.join(";"),
        modfile = par_modlist.to_string_lossy()
    );

    // launch HC and null stdin, out and error, to fork and disown process. We should be able to close launcher without killing server
    Command::new(server_launch_string)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;

    Ok(())
}

pub fn launch_hc(a3root: PathBuf, a3_executable: PathBuf, port: u64, index: u8) -> Result<()> {
    // get server password as we need to pass it to HC
    let server_password = get_server_password_from_config(find_config(&a3root)?)?;

    // build HC launch string and launch process....
    let hc_launch_string = format!(
        "{server_exe:?} -client -port={port}, -password={password}, -profiles={profile} -name=hc{index} -par={modfile}",
        server_exe = a3_executable.to_string_lossy(),
        port = port,
        password = server_password,
        profile = a3root
            .clone()
            .join(format!("headlessProfile{}", index))
            .to_string_lossy(),
        index = index,
        modfile = a3root.join(LOADED_MODS_FILE).to_string_lossy()
    );

    // launch HC and null stdin, out and error, to fork and disown process. We should be able to close launcher without killing hcs
    Command::new(hc_launch_string)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;

    Ok(())
}

/// Find the .cfg file in the A3Root to parse it and get the password
pub fn find_config(a3root: &PathBuf) -> Result<PathBuf> {
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
        Some(keys) => Ok(keys.path()),
        None => Err(anyhow::Error::msg("A3 Server Config not found in A3Root")),
    }
}

/// Given the path to the A3 server config file, extract the password for use in HCs launch parameters
pub fn get_server_password_from_config(a3_config: PathBuf) -> Result<String> {
    // find the password which is given in the file as: password = "thepassword";
    let config_string = std::fs::read_to_string(a3_config)?;

    // find and extract the password string from: password = "thepassword";
    for l in config_string.lines() {
        if l.starts_with("password") {
            if let Some((password, _)) =
                l[8..].replacen("=", "", 1).replace('"', "").split_once(";")
            {
                return Ok(password.trim().to_string());
            }
        }
    }

    Err(anyhow::Error::msg(
        "Failed to parse config and find password...",
    ))
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_config_parse() {
        // test parsing config. Give path to our dummy config for testing
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/test_asset_server.cfg");

        let server_password = get_server_password_from_config(path).unwrap();

        assert_eq!(server_password, "passwordcanbe=anything");
    }
}

// example of HC and server launch commands
//  D:\ArmaServer\A3Master\arma3server_x64.exe -client -port=2302  -password=Ariadne -profiles=D:\ArmaServer\A3Master\headlessProfile -name=hc2 -par="D:\ArmaServer\A3Files\modsSOG+.txt"
//  D:\ArmaServer\A3Master\arma3server_x64.exe -client -port=2302  -password=Ariadne -profiles=D:\ArmaServer\A3Master\headlessProfile -name=hc1 -par="D:\ArmaServer\A3Files\modsSOG+.txt"
// server launch string
//  launchModpack.ps1 -servermods "D:\ArmaServer\A3Files\modsMW25.txt" -clientmods "D:\ArmaServer\A3Files\modsCLIENT25.txt" -sprofile "serverProfile"
// launch command in script
// Start-Process -NoNewWindow -FilePath "D:\ArmaServer\A3Master\arma3server_x64.exe" -ArgumentList ("
//         -port=2302
//         -hugepages
//         -maxMem=30000
//         -maxFileCacheSize=8192
//         -enableHT
//         -bandwidthAlg=2
//         -limitFPS=1000
//         -loadMissionToMemory
//         -profiles=D:\ArmaServer\A3Master\$sprofile
//         -name=server
//         -config=D:\ArmaServer\A3Master\server.cfg
//         -cfg=$a3root\$serverprofile\Users\server\Arma3.cfg    => D:\ArmaServer\A3Master\serverProfile\Users\server\Arma3.cfg
//         -world=empty
//         -serverMod=`"@ocap;mods\@Advanced Sling Loading`"
//         -par="+$servermods)
