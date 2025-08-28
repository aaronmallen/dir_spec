use std::{env, path::PathBuf};

use crate::xdg;

pub fn bin_home() -> Option<PathBuf> {
  xdg::resolve_path_with_fallback(xdg::BIN_HOME, ".local/bin")
}

pub fn cache_home() -> Option<PathBuf> {
  xdg::resolve_path_with_fallback(xdg::CACHE_HOME, ".cache")
}

pub fn config_home() -> Option<PathBuf> {
  xdg::resolve_path_with_fallback(xdg::CONFIG_HOME, ".config")
}

pub fn config_local() -> Option<PathBuf> {
  config_home()
}

pub fn data_home() -> Option<PathBuf> {
  xdg::resolve_path_with_fallback(xdg::DATA_HOME, ".local/share")
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
  env::home_dir().map(|p| p.join(".local/share/fonts"))
}

pub fn music() -> Option<PathBuf> {
  xdg::resolve_path_with_fallback(xdg::MUSIC_DIR, "Music")
}

pub fn pictures() -> Option<PathBuf> {
  xdg::resolve_path_with_fallback(xdg::PICTURES_DIR, "Pictures")
}

pub fn preferences() -> Option<PathBuf> {
  config_home()
}

pub fn publicshare() -> Option<PathBuf> {
  xdg::resolve_path_with_fallback(xdg::PUBLICSHARE_DIR, "Public")
}

pub fn runtime() -> Option<PathBuf> {
  xdg::resolve_path(xdg::RUNTIME_DIR)
    .or_else(|| env::var("TMPDIR").ok().map(PathBuf::from).or_else(|| Some(PathBuf::from("/tmp"))))
}

pub fn state_home() -> Option<PathBuf> {
  xdg::resolve_path_with_fallback(xdg::STATE_HOME, ".local/state")
}

pub fn templates() -> Option<PathBuf> {
  xdg::resolve_path_with_fallback(xdg::TEMPLATES_DIR, "Templates")
}

pub fn videos() -> Option<PathBuf> {
  xdg::resolve_path_with_fallback(xdg::VIDEOS_DIR, "Videos")
}
