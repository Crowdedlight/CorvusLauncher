use std::path::PathBuf;

#[derive(Debug)]
pub struct ServerModList {
    pub name: String,
    pub path: PathBuf,
    pub selected: bool
}