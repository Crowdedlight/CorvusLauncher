use std::fs;
use std::fs::read_to_string;
use std::path::PathBuf;
use anyhow::Result;

// example of the input txt file we expect. All given with relative path from a3root, hence why only workshop mods have "mods\" in them.
// vn;mods\ace;mods\zen;...

/// preset for the modlists we give to server. They are comma separated path to mods from A3Master folder
///  Idea is that the selected presets, can be combined into one, adding the "-mods=" and then list of mods into a known file, that is then given to server as PAR
///  Reason is that putting all the mods on the commandline will likely hit the limit
pub(crate) struct Preset {
    pub mods: Vec<PathBuf>
}

impl Preset {
    fn new(path: PathBuf) -> Result<Preset> {

        // parse file with given path
        let raw_file = read_to_string(&path)?;
        let mods: Vec<PathBuf> = raw_file.split(";").map(PathBuf::from).collect();

        Ok(Self { mods })
    }
}

