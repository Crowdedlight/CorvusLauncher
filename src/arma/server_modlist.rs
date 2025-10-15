use std::fs;
use std::fs::read_to_string;
use std::path::PathBuf;

#[derive(Debug)]
pub struct ServerModList {
    pub name: String,
    pub path: PathBuf,
    pub selected: bool,
    pub mods: Vec<PathBuf>,
}

impl ServerModList {
    pub fn new(name: String, path: PathBuf, selected: bool) -> ServerModList {

        // parse file with given path
        let raw_file = read_to_string(&path).expect("File not found");
        let mods: Vec<PathBuf> = raw_file.split_terminator(";").map(PathBuf::from).collect();

        Self {
            name,
            path,
            selected,
            mods,
        }
    }
}

/// function to load modlists from folder. Returns vector of modlists
pub fn load_modlists(folder: &PathBuf) -> Vec<ServerModList> {

    let mut modlists: Vec<ServerModList> = Vec::new();

    // parse files if we can
    if let Ok(dir) = fs::read_dir(folder) {
        // flatten means we only get the OK results
        for entry in dir.into_iter().flatten() {

            // get path
            let path = entry.path();
            // move on if folder is here by some mistake
            if path.is_dir() {continue};

            // push to vec
            modlists.push(ServerModList::new(String::from(path.file_stem().unwrap().to_string_lossy()), path, false))
        }
    }

    modlists
}