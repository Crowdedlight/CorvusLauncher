use std::path::PathBuf;

#[derive(Debug)]
pub struct ServerModList {
    pub name: String,
    pub path: PathBuf,
    pub selected: bool
}

impl ServerModList {
    pub fn new(name: String, path: PathBuf, selected: bool) -> ServerModList {
        Self {name, path, selected}
    }
}