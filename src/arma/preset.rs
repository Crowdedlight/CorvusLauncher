use std::fs;
use std::fs::read_to_string;
use std::path::PathBuf;
use anyhow::Result;

// TODO preset for the modlists we give to server. They are comma separated path to mods from A3Master folder
//  Idea is that the selected presets, can be combined into one, adding the "-mods=" and then list of mods into a known file, that is then given to server as PAR
//  Reason is that putting all the mods on the commandline will likely hit the limit


struct Preset {
    mods: Vec<PathBuf>
}

impl Preset {
    fn new(path: PathBuf) -> Result<Preset> {

        // parse file with given path
        let raw_file = read_to_string(&path)?;
        let mods: Vec<PathBuf> = raw_file.split(";").map(PathBuf::from).collect();

        Ok(Self { mods })
    }
}

pub fn build_mods_launch_file(mods: Vec<PathBuf>, output_file: PathBuf) -> Result<()> {

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
    log::debug!("Build server modlist successfully and saved to: {:?}", output_file);

    Ok(())
}