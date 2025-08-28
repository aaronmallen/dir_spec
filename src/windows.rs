use std::{env, path::PathBuf};

use crate::xdg;

const APPDATA: &str = "APPDATA";
const LOCALAPPDATA: &str = "LOCALAPPDATA";
const USERPROFILE: &str = "USERPROFILE";

pub fn bin_home() -> Option<PathBuf> {
  resolve_path(LOCALAPPDATA).map(|p| p.join("Programs"))
}

pub fn cache_home() -> Option<PathBuf> {
  resolve_xdg_path_with_fallback(xdg::CACHE_HOME, LOCALAPPDATA)
}

pub fn config_home() -> Option<PathBuf> {
  resolve_xdg_path_with_fallback(xdg::CONFIG_HOME, APPDATA)
}

pub fn config_local() -> Option<PathBuf> {
  resolve_path(LOCALAPPDATA)
}

pub fn data_home() -> Option<PathBuf> {
  resolve_xdg_path_with_fallback(xdg::DATA_HOME, APPDATA)
}

pub fn data_local() -> Option<PathBuf> {
  env::var_os(LOCALAPPDATA).map(PathBuf::from)
}

pub fn desktop() -> Option<PathBuf> {
  resolve_xdg_path_with_fallback_and_sub_dir(xdg::DESKTOP_DIR, USERPROFILE, "Desktop")
}

pub fn documents() -> Option<PathBuf> {
  resolve_xdg_path_with_fallback_and_sub_dir(xdg::DOCUMENTS_DIR, USERPROFILE, "Documents")
}

pub fn downloads() -> Option<PathBuf> {
  resolve_xdg_path_with_fallback_and_sub_dir(xdg::DOWNLOAD_DIR, USERPROFILE, "Downloads")
}

pub fn fonts() -> Option<PathBuf> {
  None
}

pub fn music() -> Option<PathBuf> {
  resolve_xdg_path_with_fallback_and_sub_dir(xdg::MUSIC_DIR, USERPROFILE, "Music")
}

pub fn pictures() -> Option<PathBuf> {
  resolve_xdg_path_with_fallback_and_sub_dir(xdg::PICTURES_DIR, USERPROFILE, "Pictures")
}

pub fn preferences() -> Option<PathBuf> {
  config_home()
}

pub fn publicshare() -> Option<PathBuf> {
  Some(PathBuf::from("C:\\Users\\Public"))
}

pub fn runtime() -> Option<PathBuf> {
  env::var_os("TEMP").map(PathBuf::from)
}

pub fn state_home() -> Option<PathBuf> {
  resolve_xdg_path_with_fallback(xdg::STATE_HOME, LOCALAPPDATA)
}

pub fn templates() -> Option<PathBuf> {
  resolve_xdg_path_with_fallback_and_sub_dir(xdg::TEMPLATES_DIR, USERPROFILE, "Templates")
}

pub fn videos() -> Option<PathBuf> {
  resolve_xdg_path_with_fallback_and_sub_dir(xdg::VIDEOS_DIR, USERPROFILE, "Videos")
}

fn resolve_path(key: &str) -> Option<PathBuf> {
  env::var_os(key).map(PathBuf::from)
}

fn resolve_xdg_path_with_fallback(xdg_key: &str, key: &str) -> Option<PathBuf> {
  xdg::resolve_path(xdg_key).or_else(|| env::var_os(key).map(PathBuf::from))
}

fn resolve_xdg_path_with_fallback_and_sub_dir(xdg_key: &str, key: &str, sub_dir: &str) -> Option<PathBuf> {
  resolve_xdg_path_with_fallback(xdg_key, key).map(|p| p.join(sub_dir))
}
