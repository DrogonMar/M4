use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct M4Settings {
    pub game_dir: String,
}


impl M4Settings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn save_settings(path: PathBuf, settings: &M4Settings) {
        let mut file = File::create(path).expect("Failed to create settings file");
        let data = toml::to_string(&settings).expect("Failed to serialize settings");
        file.write_all(data.as_bytes()).expect("Failed to write settings to file");
        file.flush().expect("Failed to flush settings to file");
    }
}

#[derive(Debug, Clone)]
pub enum LoadErrorType {
    FileNotFound,
    CantOpenFile,
}

impl Display for LoadErrorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            LoadErrorType::FileNotFound => write!(f, "File not found"),
            LoadErrorType::CantOpenFile => write!(f, "Cannot open file"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LoadError(pub(crate) LoadErrorType);

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to load settings: {}", self.0)
    }
}
