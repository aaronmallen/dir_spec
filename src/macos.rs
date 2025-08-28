use std::{env, path::PathBuf};

use crate::xdg;

const APP_SUPPORT: &str = "Library/Application Support";

pub fn bin_home() -> Option<PathBuf> {
  xdg::resolve_path_with_fallback(xdg::BIN_HOME, ".local/bin")
}

pub fn cache_home() -> Option<PathBuf> {
  xdg::resolve_path_with_fallback(xdg::CACHE_HOME, "Library/Caches")
}

pub fn config_home() -> Option<PathBuf> {
  xdg::resolve_path_with_fallback(xdg::CONFIG_HOME, APP_SUPPORT)
}

pub fn config_local() -> Option<PathBuf> {
  config_home()
}

pub fn data_home() -> Option<PathBuf> {
  xdg::resolve_path_with_fallback(xdg::DATA_HOME, APP_SUPPORT)
}

pub fn data_local() -> Option<PathBuf> {
  data_home()
}

pub fn desktop() -> Option<PathBuf> {
  xdg::resolve_path_with_fallback(xdg::DESKTOP_DIR, "Desktop")
}

pub fn documents() -> Option<PathBuf> {
  xdg::resolve_path_with_fallback(xdg::DOCUMENTS_DIR, "Documents")
}

pub fn downloads() -> Option<PathBuf> {
  xdg::resolve_path_with_fallback(xdg::DOWNLOAD_DIR, "Downloads")
}

pub fn fonts() -> Option<PathBuf> {
  env::home_dir().map(|p| p.join("Library/Fonts"))
}

pub fn music() -> Option<PathBuf> {
  xdg::resolve_path_with_fallback(xdg::MUSIC_DIR, "Music")
}

pub fn pictures() -> Option<PathBuf> {
  xdg::resolve_path_with_fallback(xdg::PICTURES_DIR, "Pictures")
}

pub fn preferences() -> Option<PathBuf> {
  env::home_dir().map(|p| p.join("Library/Preferences"))
}

pub fn publicshare() -> Option<PathBuf> {
  xdg::resolve_path_with_fallback(xdg::PUBLICSHARE_DIR, "Public")
}

pub fn runtime() -> Option<PathBuf> {
  xdg::resolve_path(xdg::RUNTIME_DIR)
    .or_else(|| env::var("TMPDIR").ok().map(PathBuf::from).or_else(|| Some(PathBuf::from("/tmp"))))
}

pub fn state_home() -> Option<PathBuf> {
  xdg::resolve_path_with_fallback(xdg::STATE_HOME, APP_SUPPORT)
}

pub fn templates() -> Option<PathBuf> {
  xdg::resolve_path_with_fallback(xdg::TEMPLATES_DIR, "Templates")
}

pub fn videos() -> Option<PathBuf> {
  xdg::resolve_path_with_fallback(xdg::VIDEOS_DIR, "Movies")
}
