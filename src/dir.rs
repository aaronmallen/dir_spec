use std::{env, path::PathBuf};

use eyre::Result;

pub struct Dir;

impl Dir {
  pub fn bin_home() -> Result<PathBuf> {
    if let Ok(xdg_bin_home) = env::var("XDG_BIN_HOME") {
      return Ok(PathBuf::from(xdg_bin_home));
    }

    #[cfg(target_os = "macos")]
    {
      Ok(Self::home()?.join(".local/bin"))
    }

    #[cfg(target_os = "windows")]
    {
      if let Ok(localappdata) = env::var("LOCALAPPDATA") {
        Ok(PathBuf::from(localappdata).join("Programs"))
      } else {
        Err(eyre::eyre!("Failed to resolve bin directory path"))
      }
    }

    #[cfg(target_os = "linux")]
    {
      Ok(Self::home()?.join(".local/bin"))
    }
  }

  pub fn cache_home() -> Result<PathBuf> {
    if let Ok(xdg_cache_home) = env::var("XDG_CACHE_HOME") {
      return Ok(PathBuf::from(xdg_cache_home));
    }

    #[cfg(target_os = "macos")]
    {
      Ok(Self::home()?.join("Library/Caches"))
    }

    #[cfg(target_os = "windows")]
    {
      if let Ok(localappdata) = env::var("LOCALAPPDATA") {
        Ok(PathBuf::from(localappdata))
      } else {
        Err(eyre::eyre!("Failed to resolve cache directory"))
      }
    }

    #[cfg(target_os = "linux")]
    {
      Ok(Self::home()?.join(".cache"))
    }
  }

  pub fn config_home() -> Result<PathBuf> {
    if let Ok(xdg_config_home) = env::var("XDG_CONFIG_HOME") {
      return Ok(PathBuf::from(xdg_config_home));
    }

    #[cfg(target_os = "macos")]
    {
      Ok(Self::home()?.join("Library/Application Support"))
    }

    #[cfg(target_os = "windows")]
    {
      if let Ok(appdata) = env::var("APPDATA") {
        Ok(PathBuf::from(appdata))
      } else {
        Err(eyre::eyre!("Failed to resolve config directory"))
      }
    }

    #[cfg(target_os = "linux")]
    {
      Ok(Self::home()?.join(".config"))
    }
  }

  pub fn data_home() -> Result<PathBuf> {
    if let Ok(xdg_data_home) = env::var("XDG_DATA_HOME") {
      return Ok(PathBuf::from(xdg_data_home));
    }

    #[cfg(target_os = "macos")]
    {
      Ok(Self::home()?.join("Library/Application Support"))
    }

    #[cfg(target_os = "windows")]
    {
      if let Ok(appdata) = env::var("APPDATA") {
        Ok(PathBuf::from(appdata))
      } else {
        Err(eyre::eyre!("Failed to resolve data directory"))
      }
    }

    #[cfg(target_os = "linux")]
    {
      Ok(Self::home()?.join(".local/share"))
    }
  }

  pub fn desktop_dir() -> Result<PathBuf> {
    if let Ok(xdg_desktop_dir) = env::var("XDG_DESKTOP_DIR") {
      return Ok(PathBuf::from(xdg_desktop_dir));
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    {
      Ok(Self::home()?.join("Desktop"))
    }

    #[cfg(target_os = "windows")]
    {
      if let Ok(userprofile) = env::var("USERPROFILE") {
        Ok(PathBuf::from(userprofile).join("Desktop"))
      } else {
        Err(eyre::eyre!("Failed to resolve desktop directory"))
      }
    }
  }

  pub fn documents_dir() -> Result<PathBuf> {
    if let Ok(xdg_documents_dir) = env::var("XDG_DOCUMENTS_DIR") {
      return Ok(PathBuf::from(xdg_documents_dir));
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    {
      Ok(Self::home()?.join("Documents"))
    }

    #[cfg(target_os = "windows")]
    {
      if let Ok(userprofile) = env::var("USERPROFILE") {
        Ok(PathBuf::from(userprofile).join("Documents"))
      } else {
        Err(eyre::eyre!("Failed to resolve documents directory"))
      }
    }
  }

  pub fn download_dir() -> Result<PathBuf> {
    if let Ok(xdg_download_dir) = env::var("XDG_DOWNLOAD_DIR") {
      return Ok(PathBuf::from(xdg_download_dir));
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    {
      Ok(Self::home()?.join("Downloads"))
    }

    #[cfg(target_os = "windows")]
    {
      if let Ok(userprofile) = env::var("USERPROFILE") {
        Ok(PathBuf::from(userprofile).join("Downloads"))
      } else {
        Err(eyre::eyre!("Failed to resolve download directory"))
      }
    }
  }

  pub fn home() -> Result<PathBuf> {
    #[cfg(unix)]
    {
      if let Ok(home) = env::var("HOME") {
        return Ok(PathBuf::from(home));
      }

      let uid = unsafe { libc::getuid() };
      let passwd = unsafe { libc::getpwuid(uid) };

      if passwd.is_null() {
        return Err(eyre::eyre!("Failed to resolve the home directory path"));
      }

      let home_cstr = unsafe { std::ffi::CStr::from_ptr((*passwd).pw_dir) };
      let home_str = home_cstr.to_str().map_err(|_| eyre::eyre!("Failed to resolve the home directory path"))?;

      Ok(PathBuf::from(home_str))
    }

    #[cfg(windows)]
    {
      if let Ok(profile) = env::var("USERPROFILE") {
        return Ok(PathBuf::from(profile));
      }

      if let (Ok(drive), Ok(path)) = (env::var("HOMEDRIVE"), env::var("HOMEPATH")) {
        return Ok(PathBuf::from(format!("{}{}", drive, path)));
      }

      Err(eyre::eyre!("Failed to resolve the home directory path"))
    }

    #[cfg(not(any(unix, windows)))]
    {
      Err(eyre::eyre!("Failed to resolve the home directory path: Unsupported platform"))
    }
  }

  pub fn music_dir() -> Result<PathBuf> {
    if let Ok(xdg_music_dir) = env::var("XDG_MUSIC_DIR") {
      return Ok(PathBuf::from(xdg_music_dir));
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    {
      Ok(Self::home()?.join("Music"))
    }

    #[cfg(target_os = "windows")]
    {
      if let Ok(userprofile) = env::var("USERPROFILE") {
        Ok(PathBuf::from(userprofile).join("Music"))
      } else {
        Err(eyre::eyre!("Failed to resolve music directory"))
      }
    }
  }

  pub fn pictures_dir() -> Result<PathBuf> {
    if let Ok(xdg_pictures_dir) = env::var("XDG_PICTURES_DIR") {
      return Ok(PathBuf::from(xdg_pictures_dir));
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    {
      Ok(Self::home()?.join("Pictures"))
    }

    #[cfg(target_os = "windows")]
    {
      if let Ok(userprofile) = env::var("USERPROFILE") {
        Ok(PathBuf::from(userprofile).join("Pictures"))
      } else {
        Err(eyre::eyre!("Failed to resolve pictures directory"))
      }
    }
  }

  pub fn publicshare_dir() -> Result<PathBuf> {
    if let Ok(xdg_publicshare_dir) = env::var("XDG_PUBLICSHARE_DIR") {
      return Ok(PathBuf::from(xdg_publicshare_dir));
    }

    #[cfg(target_os = "macos")]
    {
      Ok(Self::home()?.join("Public"))
    }

    #[cfg(target_os = "windows")]
    {
      Ok(PathBuf::from("C:\\Users\\Public"))
    }

    #[cfg(target_os = "linux")]
    {
      Ok(Self::home()?.join("Public"))
    }
  }

  pub fn runtime_dir() -> Result<PathBuf> {
    if let Ok(xdg_runtime_dir) = env::var("XDG_RUNTIME_DIR") {
      return Ok(PathBuf::from(xdg_runtime_dir));
    }

    #[cfg(target_os = "macos")]
    {
      if let Ok(tmpdir) = env::var("TMPDIR") { Ok(PathBuf::from(tmpdir)) } else { Ok(PathBuf::from("/tmp")) }
    }

    #[cfg(target_os = "windows")]
    {
      if let Ok(temp) = env::var("TEMP") {
        Ok(PathBuf::from(temp))
      } else {
        Err(eyre::eyre!("Failed to resolve runtime directory"))
      }
    }

    #[cfg(target_os = "linux")]
    {
      let uid = unsafe { libc::getuid() };
      Ok(PathBuf::from(format!("/run/user/{}", uid)))
    }
  }

  pub fn state_home() -> Result<PathBuf> {
    if let Ok(xdg_state_home) = env::var("XDG_STATE_HOME") {
      return Ok(PathBuf::from(xdg_state_home));
    }

    #[cfg(target_os = "macos")]
    {
      Ok(Self::home()?.join("Library/Application Support"))
    }

    #[cfg(target_os = "windows")]
    {
      if let Ok(localappdata) = env::var("LOCALAPPDATA") {
        Ok(PathBuf::from(localappdata))
      } else {
        Err(eyre::eyre!("Failed to resolve state directory"))
      }
    }

    #[cfg(target_os = "linux")]
    {
      Ok(Self::home()?.join(".local/state"))
    }
  }

  pub fn templates_dir() -> Result<PathBuf> {
    if let Ok(xdg_templates_dir) = env::var("XDG_TEMPLATES_DIR") {
      return Ok(PathBuf::from(xdg_templates_dir));
    }

    #[cfg(target_os = "macos")]
    {
      Ok(Self::home()?.join("Templates"))
    }

    #[cfg(target_os = "windows")]
    {
      if let Ok(userprofile) = env::var("USERPROFILE") {
        Ok(PathBuf::from(userprofile).join("Templates"))
      } else {
        Err(eyre::eyre!("Failed to resolve templates directory"))
      }
    }

    #[cfg(target_os = "linux")]
    {
      Ok(Self::home()?.join("Templates"))
    }
  }

  pub fn videos_dir() -> Result<PathBuf> {
    if let Ok(xdg_videos_dir) = env::var("XDG_VIDEOS_DIR") {
      return Ok(PathBuf::from(xdg_videos_dir));
    }

    #[cfg(target_os = "macos")]
    {
      Ok(Self::home()?.join("Movies"))
    }

    #[cfg(target_os = "windows")]
    {
      if let Ok(userprofile) = env::var("USERPROFILE") {
        Ok(PathBuf::from(userprofile).join("Videos"))
      } else {
        Err(eyre::eyre!("Failed to resolve videos directory"))
      }
    }

    #[cfg(target_os = "linux")]
    {
      Ok(Self::home()?.join("Videos"))
    }
  }
}
