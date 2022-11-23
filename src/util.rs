use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use serde::Deserialize;
use crate::constants;
use crate::m4_settings::{LoadError, LoadErrorType};

#[cfg(target_os = "linux")]
pub fn get_config_path() -> Option<PathBuf> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix(constants::APPNAME).unwrap();
    let setting_file = xdg_dirs.place_config_file(constants::SETTINGS_FILE).expect("Cannot create config file");
    Some(setting_file)
}

#[cfg(target_os = "windows")]
pub fn get_config_path() -> Option<PathBuf> {
    //%AppData%\VrMote\settings.toml
    let mut path = std::env::var("AppData").expect("Cannot get APPDATA environment variable");
    path.push_str("\\");
    path.push_str(constants::APPNAME);
    //Create the directory if it doesn't exist
    std::fs::create_dir_all(&path).expect("Cannot create config directory");

    //Now add the file name
    path.push_str("\\");
    path.push_str(constants::SETTINGS_FILE);
    println!("Config path: {}", path);
    Some(PathBuf::from(path))
}

#[cfg(target_family = "wasm")]
pub fn get_config_path() -> Option<PathBuf>{
    let mut path = PathBuf::from("settings.toml");
    Some(path)
}

pub fn is_valid_megamix_dir(path: &Path) -> bool {
    if !path.exists() {
        return false;
    }

    if !path.to_path_buf().join("DivaMegaMix.exe").exists() {
        return false;
    }

    return true;
}

pub fn has_mod_loader(path: &Path) -> bool {
    if !is_valid_megamix_dir(path){
        return false;
    }

    if !path.to_path_buf().join("dinput8.dll").exists() {
        return false;
    }

    return true;
}

pub async fn load_file_into<T: for<'a> Deserialize<'a>>(path: PathBuf) -> Result<T, LoadError> {
    if !Path::new(&path).exists() {
        return Err(LoadError(LoadErrorType::FileNotFound));
    }

    let conf_file = File::open(&path);
    if conf_file.is_err() {
        return Err(LoadError(LoadErrorType::CantOpenFile));
    }

    let mut buffer = String::new();
    conf_file.unwrap().read_to_string(&mut buffer).expect("Failed to read file");

    let settings: T = toml::from_str(&buffer).expect("Failed to parse settings");

    Ok(settings)
}