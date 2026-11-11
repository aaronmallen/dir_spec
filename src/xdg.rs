use std::{env, path::PathBuf};

pub const BIN_HOME: &str = "XDG_BIN_HOME";
pub const CACHE_HOME: &str = "XDG_CACHE_HOME";
pub const CONFIG_HOME: &str = "XDG_CONFIG_HOME";
pub const DATA_HOME: &str = "XDG_DATA_HOME";
pub const DESKTOP_DIR: &str = "XDG_DESKTOP_DIR";
pub const DOCUMENTS_DIR: &str = "XDG_DOCUMENTS_DIR";
pub const DOWNLOAD_DIR: &str = "XDG_DOWNLOAD_DIR";
pub const MUSIC_DIR: &str = "XDG_MUSIC_DIR";
pub const PICTURES_DIR: &str = "XDG_PICTURES_DIR";
pub const PUBLICSHARE_DIR: &str = "XDG_PUBLICSHARE_DIR";
pub const RUNTIME_DIR: &str = "XDG_RUNTIME_DIR";
pub const STATE_HOME: &str = "XDG_STATE_HOME";
pub const TEMPLATES_DIR: &str = "XDG_TEMPLATES_DIR";
pub const VIDEOS_DIR: &str = "XDG_VIDEOS_DIR";

pub fn resolve_path(key: &str) -> Option<PathBuf> {
  env::var_os(key).map(PathBuf::from).filter(|p| p.is_absolute())
}

pub fn resolve_path_with_fallback(key: &str, default: &str) -> Option<PathBuf> {
  resolve_path(key).or_else(|| env::home_dir().map(|p| p.join(default)))
}
