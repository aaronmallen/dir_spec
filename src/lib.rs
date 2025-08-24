use std::{env, path::PathBuf};

/// Cross-platform directory resolver with XDG compliance.
///
/// This struct provides methods to resolve standard directories across Linux, macOS, and Windows
/// while respecting XDG Base Directory Specification environment variables when set.
///
/// # XDG Compliance
///
/// All methods check for corresponding XDG environment variables first (e.g., `XDG_CONFIG_HOME`),
/// and only fall back to platform-specific defaults if the XDG variable is not set or contains
/// a relative path (which the XDG spec requires to be ignored).
///
/// # Examples
///
/// ```rust
/// use dir_spec::Dir;
///
/// // Get config directory (respects XDG_CONFIG_HOME if set)
/// if let Some(config) = Dir::config_home() {
///     println!("Config directory: {}", config.display());
/// }
///
/// // Get cache directory (respects XDG_CACHE_HOME if set)
/// if let Some(cache) = Dir::cache_home() {
///     println!("Cache directory: {}", cache.display());
/// }
///
/// // Get videos directory (new cleaner API)
/// if let Some(videos) = Dir::videos() {
///     println!("Videos directory: {}", videos.display());
/// }
/// ```
pub struct Dir;

impl Dir {
  /// Returns the user's binary directory.
  ///
  /// Checks `XDG_BIN_HOME` first, then falls back to platform defaults:
  /// - **Linux/macOS**: `~/.local/bin`
  /// - **Windows**: `%LOCALAPPDATA%\Programs`
  ///
  /// # Examples
  ///
  /// ```rust
  /// use dir_spec::Dir;
  /// if let Some(bin_dir) = Dir::bin_home() {
  ///     println!("Bin directory: {}", bin_dir.display());
  /// }
  /// ```
  pub fn bin_home() -> Option<PathBuf> {
    if let Some(path) = Self::resolve_xdg_path("XDG_BIN_HOME") {
      return Some(path);
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    {
      Self::home().map(|home| home.join(".local/bin"))
    }

    #[cfg(target_os = "windows")]
    {
      env::var("LOCALAPPDATA").ok().map(|path| PathBuf::from(path).join("Programs"))
    }
  }

  /// Returns the user's cache directory.
  ///
  /// Checks `XDG_CACHE_HOME` first, then falls back to platform defaults:
  /// - **Linux**: `~/.cache`
  /// - **macOS**: `~/Library/Caches`
  /// - **Windows**: `%LOCALAPPDATA%`
  ///
  /// # Examples
  ///
  /// ```rust
  /// use dir_spec::Dir;
  /// if let Some(cache_dir) = Dir::cache_home() {
  ///     println!("Cache directory: {}", cache_dir.display());
  /// }
  /// ```
  pub fn cache_home() -> Option<PathBuf> {
    Self::resolve_xdg_path("XDG_CACHE_HOME")
      .or_else(|| Self::get_platform_default("Library/Caches", "LOCALAPPDATA", ".cache"))
  }

  /// Returns the user's configuration directory.
  ///
  /// Checks `XDG_CONFIG_HOME` first, then falls back to platform defaults:
  /// - **Linux**: `~/.config`
  /// - **macOS**: `~/Library/Application Support`
  /// - **Windows**: `%APPDATA%`
  ///
  /// # Examples
  ///
  /// ```rust
  /// use dir_spec::Dir;
  /// if let Some(config_dir) = Dir::config_home() {
  ///     println!("Config directory: {}", config_dir.display());
  /// }
  /// ```
  pub fn config_home() -> Option<PathBuf> {
    Self::resolve_xdg_path("XDG_CONFIG_HOME")
      .or_else(|| Self::get_platform_default("Library/Application Support", "APPDATA", ".config"))
  }

  /// Returns the user's local configuration directory (non-roaming).
  ///
  /// This is primarily useful on Windows where it returns the local (non-roaming) config directory.
  /// On other platforms, it behaves identically to `config_home()`.
  ///
  /// Platform defaults:
  /// - **Linux**: `~/.config` (same as `config_home()`)
  /// - **macOS**: `~/Library/Application Support` (same as `config_home()`)
  /// - **Windows**: `%LOCALAPPDATA%` (non-roaming)
  ///
  /// # Examples
  ///
  /// ```rust
  /// use dir_spec::Dir;
  /// if let Some(config_local) = Dir::config_local() {
  ///     println!("Local config directory: {}", config_local.display());
  /// }
  /// ```
  pub fn config_local() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    {
      env::var("LOCALAPPDATA").ok().map(PathBuf::from)
    }

    #[cfg(not(target_os = "windows"))]
    {
      Self::config_home()
    }
  }

  /// Returns the user's data directory.
  ///
  /// Checks `XDG_DATA_HOME` first, then falls back to platform defaults:
  /// - **Linux**: `~/.local/share`
  /// - **macOS**: `~/Library/Application Support`
  /// - **Windows**: `%APPDATA%`
  ///
  /// # Examples
  ///
  /// ```rust
  /// use dir_spec::Dir;
  /// if let Some(data_dir) = Dir::data_home() {
  ///     println!("Data directory: {}", data_dir.display());
  /// }
  /// ```
  pub fn data_home() -> Option<PathBuf> {
    Self::resolve_xdg_path("XDG_DATA_HOME")
      .or_else(|| Self::get_platform_default("Library/Application Support", "APPDATA", ".local/share"))
  }

  /// Returns the user's local data directory (non-roaming).
  ///
  /// This is primarily useful on Windows where it returns the local (non-roaming) data directory.
  /// On other platforms, it behaves identically to `data_home()`.
  ///
  /// Platform defaults:
  /// - **Linux**: `~/.local/share` (same as `data_home()`)
  /// - **macOS**: `~/Library/Application Support` (same as `data_home()`)
  /// - **Windows**: `%LOCALAPPDATA%` (non-roaming)
  ///
  /// # Examples
  ///
  /// ```rust
  /// use dir_spec::Dir;
  /// if let Some(data_local) = Dir::data_local() {
  ///     println!("Local data directory: {}", data_local.display());
  /// }
  /// ```
  pub fn data_local() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    {
      env::var("LOCALAPPDATA").ok().map(PathBuf::from)
    }

    #[cfg(not(target_os = "windows"))]
    {
      Self::data_home()
    }
  }

  /// Returns the user's desktop directory.
  ///
  /// Checks `XDG_DESKTOP_DIR` first, then falls back to platform defaults:
  /// - **Linux/macOS**: `~/Desktop`
  /// - **Windows**: `%USERPROFILE%\Desktop`
  ///
  /// # Examples
  ///
  /// ```rust
  /// use dir_spec::Dir;
  /// if let Some(desktop) = Dir::desktop() {
  ///     println!("Desktop directory: {}", desktop.display());
  /// }
  /// ```
  pub fn desktop() -> Option<PathBuf> {
    Self::resolve_xdg_path("XDG_DESKTOP_DIR")
      .or_else(|| Self::get_platform_default_with_windows_subdir("Desktop", "USERPROFILE", "Desktop", "Desktop"))
  }

  /// Returns the user's documents directory.
  ///
  /// Checks `XDG_DOCUMENTS_DIR` first, then falls back to platform defaults:
  /// - **Linux/macOS**: `~/Documents`
  /// - **Windows**: `%USERPROFILE%\Documents`
  ///
  /// # Examples
  ///
  /// ```rust
  /// use dir_spec::Dir;
  /// if let Some(documents) = Dir::documents() {
  ///     println!("Documents directory: {}", documents.display());
  /// }
  /// ```
  pub fn documents() -> Option<PathBuf> {
    Self::resolve_xdg_path("XDG_DOCUMENTS_DIR")
      .or_else(|| Self::get_platform_default_with_windows_subdir("Documents", "USERPROFILE", "Documents", "Documents"))
  }

  /// Returns the user's downloads directory.
  ///
  /// Checks `XDG_DOWNLOAD_DIR` first, then falls back to platform defaults:
  /// - **Linux/macOS**: `~/Downloads`
  /// - **Windows**: `%USERPROFILE%\Downloads`
  ///
  /// # Examples
  ///
  /// ```rust
  /// use dir_spec::Dir;
  /// if let Some(downloads) = Dir::downloads() {
  ///     println!("Downloads directory: {}", downloads.display());
  /// }
  /// ```
  pub fn downloads() -> Option<PathBuf> {
    Self::resolve_xdg_path("XDG_DOWNLOAD_DIR")
      .or_else(|| Self::get_platform_default_with_windows_subdir("Downloads", "USERPROFILE", "Downloads", "Downloads"))
  }

  /// Returns the user's fonts directory.
  ///
  /// This directory is used for user-installed fonts.
  /// Note: Returns `None` on Windows as there is no standard user fonts directory.
  ///
  /// Platform defaults:
  /// - **Linux**: `~/.local/share/fonts`
  /// - **macOS**: `~/Library/Fonts`
  /// - **Windows**: `None` (no standard user fonts directory)
  ///
  /// # Examples
  ///
  /// ```rust
  /// use dir_spec::Dir;
  /// if let Some(fonts) = Dir::fonts() {
  ///     println!("Fonts directory: {}", fonts.display());
  /// } else {
  ///     println!("No user fonts directory on this platform");
  /// }
  /// ```
  pub fn fonts() -> Option<PathBuf> {
    #[cfg(target_os = "macos")]
    {
      Self::home().map(|home| home.join("Library/Fonts"))
    }

    #[cfg(target_os = "linux")]
    {
      Self::home().map(|home| home.join(".local/share/fonts"))
    }

    #[cfg(target_os = "windows")]
    {
      None
    }
  }

  /// Returns the user's home directory.
  ///
  /// Uses the standard library's `std::env::home_dir()` function.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use dir_spec::Dir;
  /// if let Some(home_dir) = Dir::home() {
  ///     println!("Home directory: {}", home_dir.display());
  /// }
  /// ```
  pub fn home() -> Option<PathBuf> {
    std::env::home_dir()
  }

  /// Returns the user's music directory.
  ///
  /// Checks `XDG_MUSIC_DIR` first, then falls back to platform defaults:
  /// - **Linux/macOS**: `~/Music`
  /// - **Windows**: `%USERPROFILE%\Music`
  ///
  /// # Examples
  ///
  /// ```rust
  /// use dir_spec::Dir;
  /// if let Some(music) = Dir::music() {
  ///     println!("Music directory: {}", music.display());
  /// }
  /// ```
  pub fn music() -> Option<PathBuf> {
    Self::resolve_xdg_path("XDG_MUSIC_DIR")
      .or_else(|| Self::get_platform_default_with_windows_subdir("Music", "USERPROFILE", "Music", "Music"))
  }

  /// Returns the user's pictures directory.
  ///
  /// Checks `XDG_PICTURES_DIR` first, then falls back to platform defaults:
  /// - **Linux/macOS**: `~/Pictures`
  /// - **Windows**: `%USERPROFILE%\Pictures`
  ///
  /// # Examples
  ///
  /// ```rust
  /// use dir_spec::Dir;
  /// if let Some(pictures) = Dir::pictures() {
  ///     println!("Pictures directory: {}", pictures.display());
  /// }
  /// ```
  pub fn pictures() -> Option<PathBuf> {
    Self::resolve_xdg_path("XDG_PICTURES_DIR")
      .or_else(|| Self::get_platform_default_with_windows_subdir("Pictures", "USERPROFILE", "Pictures", "Pictures"))
  }

  /// Returns the user's preferences directory.
  ///
  /// This is primarily used on macOS for storing .plist files using Apple's proprietary APIs.
  /// On other platforms, it behaves identically to `config_home()`.
  ///
  /// Platform defaults:
  /// - **Linux**: `~/.config` (same as `config_home()`)
  /// - **macOS**: `~/Library/Preferences` (for .plist files)
  /// - **Windows**: `%APPDATA%` (same as `config_home()`)
  ///
  /// # Examples
  ///
  /// ```rust
  /// use dir_spec::Dir;
  /// if let Some(preferences) = Dir::preferences() {
  ///     println!("Preferences directory: {}", preferences.display());
  /// }
  /// ```
  pub fn preferences() -> Option<PathBuf> {
    #[cfg(target_os = "macos")]
    {
      Self::home().map(|home| home.join("Library/Preferences"))
    }

    #[cfg(not(target_os = "macos"))]
    {
      Self::config_home()
    }
  }

  /// Returns the user's public share directory.
  ///
  /// Checks `XDG_PUBLICSHARE_DIR` first, then falls back to platform defaults:
  /// - **Linux/macOS**: `~/Public`
  /// - **Windows**: `C:\Users\Public` (system-wide public folder)
  ///
  /// # Examples
  ///
  /// ```rust
  /// use dir_spec::Dir;
  /// if let Some(public) = Dir::publicshare() {
  ///     println!("Public directory: {}", public.display());
  /// }
  /// ```
  pub fn publicshare() -> Option<PathBuf> {
    if let Some(path) = Self::resolve_xdg_path("XDG_PUBLICSHARE_DIR") {
      return Some(path);
    }

    #[cfg(target_os = "macos")]
    {
      Self::home().map(|home| home.join("Public"))
    }

    #[cfg(target_os = "windows")]
    {
      Some(PathBuf::from("C:\\Users\\Public"))
    }

    #[cfg(target_os = "linux")]
    {
      Self::home().map(|home| home.join("Public"))
    }
  }

  /// Returns the user's runtime directory.
  ///
  /// Checks `XDG_RUNTIME_DIR` first, then falls back to platform defaults:
  /// - **Linux**: Attempts to use `$TMPDIR`, then falls back to `/tmp`
  /// - **macOS**: `$TMPDIR` or `/tmp`
  /// - **Windows**: `%TEMP%`
  ///
  /// # Examples
  ///
  /// ```rust
  /// use dir_spec::Dir;
  /// if let Some(runtime) = Dir::runtime() {
  ///     println!("Runtime directory: {}", runtime.display());
  /// }
  /// ```
  pub fn runtime() -> Option<PathBuf> {
    if let Some(path) = Self::resolve_xdg_path("XDG_RUNTIME_DIR") {
      return Some(path);
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    {
      env::var("TMPDIR").ok().map(PathBuf::from).or_else(|| Some(PathBuf::from("/tmp")))
    }

    #[cfg(target_os = "windows")]
    {
      env::var("TEMP").ok().map(PathBuf::from)
    }
  }

  /// Returns the user's state directory.
  ///
  /// Checks `XDG_STATE_HOME` first, then falls back to platform defaults:
  /// - **Linux**: `~/.local/state`
  /// - **macOS**: `~/Library/Application Support`
  /// - **Windows**: `%LOCALAPPDATA%`
  ///
  /// # Examples
  ///
  /// ```rust
  /// use dir_spec::Dir;
  /// if let Some(state_dir) = Dir::state_home() {
  ///     println!("State directory: {}", state_dir.display());
  /// }
  /// ```
  pub fn state_home() -> Option<PathBuf> {
    Self::resolve_xdg_path("XDG_STATE_HOME")
      .or_else(|| Self::get_platform_default("Library/Application Support", "LOCALAPPDATA", ".local/state"))
  }

  /// Returns the user's templates directory.
  ///
  /// Checks `XDG_TEMPLATES_DIR` first, then falls back to platform defaults:
  /// - **Linux/macOS**: `~/Templates`
  /// - **Windows**: `%USERPROFILE%\Templates`
  ///
  /// # Examples
  ///
  /// ```rust
  /// use dir_spec::Dir;
  /// if let Some(templates) = Dir::templates() {
  ///     println!("Templates directory: {}", templates.display());
  /// }
  /// ```
  pub fn templates() -> Option<PathBuf> {
    Self::resolve_xdg_path("XDG_TEMPLATES_DIR")
      .or_else(|| Self::get_platform_default_with_windows_subdir("Templates", "USERPROFILE", "Templates", "Templates"))
  }

  /// Returns the user's videos directory.
  ///
  /// Checks `XDG_VIDEOS_DIR` first, then falls back to platform defaults:
  /// - **Linux**: `~/Videos`
  /// - **macOS**: `~/Movies` (following macOS convention)
  /// - **Windows**: `%USERPROFILE%\Videos`
  ///
  /// # Examples
  ///
  /// ```rust
  /// use dir_spec::Dir;
  /// if let Some(videos) = Dir::videos() {
  ///     println!("Videos directory: {}", videos.display());
  /// }
  /// ```
  pub fn videos() -> Option<PathBuf> {
    Self::resolve_xdg_path("XDG_VIDEOS_DIR")
      .or_else(|| Self::get_platform_default_with_windows_subdir("Movies", "USERPROFILE", "Videos", "Videos"))
  }

  /// Resolves platform-specific directory paths using a simple pattern.
  ///
  /// This helper handles the common case where:
  /// - macOS and Linux use home-relative paths
  /// - Windows uses an environment variable directly
  fn get_platform_default(
    macos_path: &str,
    #[allow(unused_variables)] windows_env: &str,
    #[allow(unused_variables)] linux_path: &str,
  ) -> Option<PathBuf> {
    #[cfg(target_os = "macos")]
    {
      Self::home().map(|home| home.join(macos_path))
    }

    #[cfg(target_os = "windows")]
    {
      env::var(windows_env).ok().map(PathBuf::from)
    }

    #[cfg(target_os = "linux")]
    {
      Self::home().map(|home| home.join(linux_path))
    }
  }

  /// Resolves platform-specific directory paths where Windows needs a subdirectory.
  ///
  /// This helper handles cases where:
  /// - macOS and Linux use home-relative paths
  /// - Windows uses an environment variable plus a subdirectory
  fn get_platform_default_with_windows_subdir(
    macos_path: &str,
    #[allow(unused_variables)] windows_env: &str,
    #[allow(unused_variables)] windows_subdir: &str,
    #[allow(unused_variables)] linux_path: &str,
  ) -> Option<PathBuf> {
    #[cfg(target_os = "macos")]
    {
      Self::home().map(|home| home.join(macos_path))
    }

    #[cfg(target_os = "windows")]
    {
      env::var(windows_env).ok().map(|path| PathBuf::from(path).join(windows_subdir))
    }

    #[cfg(target_os = "linux")]
    {
      Self::home().map(|home| home.join(linux_path))
    }
  }

  /// Resolves an XDG environment variable to an absolute path.
  ///
  /// Returns `Some(PathBuf)` if the environment variable is set and contains an absolute path,
  /// `None` otherwise. This follows the XDG Base Directory Specification requirement that
  /// all paths must be absolute.
  fn resolve_xdg_path(var: &str) -> Option<PathBuf> {
    env::var(var).ok().map(PathBuf::from).filter(|path| path.is_absolute())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  mod bin_home {
    use temp_env::{with_var, with_var_unset};

    use super::*;

    #[test]
    fn respects_xdg_bin_home() {
      let test_path = if cfg!(windows) { "C:\\test\\bin" } else { "/test/bin" };
      with_var("XDG_BIN_HOME", Some(test_path), || {
        let result = Dir::bin_home();
        assert_eq!(result, Some(PathBuf::from(test_path)));
      });
    }

    #[test]
    fn ignores_relative_xdg_bin_home() {
      with_var("XDG_BIN_HOME", Some("relative/bin"), || {
        let result = Dir::bin_home();

        if let Some(path) = result {
          assert!(path.is_absolute());
        }
      });
    }

    #[test]
    fn uses_platform_default_when_xdg_unset() {
      with_var_unset("XDG_BIN_HOME", || {
        let result = Dir::bin_home();
        if let Some(bin_path) = result {
          assert!(bin_path.is_absolute());

          #[cfg(any(target_os = "linux", target_os = "macos"))]
          assert!(bin_path.to_string_lossy().ends_with(".local/bin"));

          #[cfg(target_os = "windows")]
          assert!(bin_path.to_string_lossy().contains("Programs"));
        }
      });
    }
  }

  mod cache_home {
    use temp_env::{with_var, with_var_unset};

    use super::*;

    #[test]
    fn respects_xdg_cache_home() {
      let test_path = if cfg!(windows) { "C:\\test\\cache" } else { "/test/cache" };
      with_var("XDG_CACHE_HOME", Some(test_path), || {
        let result = Dir::cache_home();
        assert_eq!(result, Some(PathBuf::from(test_path)));
      });
    }

    #[test]
    fn ignores_relative_xdg_cache_home() {
      with_var("XDG_CACHE_HOME", Some("relative/cache"), || {
        let result = Dir::cache_home();
        if let Some(path) = result {
          assert!(path.is_absolute());
        }
      });
    }

    #[test]
    fn uses_platform_default_when_xdg_unset() {
      with_var_unset("XDG_CACHE_HOME", || {
        let result = Dir::cache_home();
        if let Some(cache_path) = result {
          assert!(cache_path.is_absolute());

          #[cfg(target_os = "linux")]
          assert!(cache_path.to_string_lossy().ends_with(".cache"));

          #[cfg(target_os = "macos")]
          assert!(cache_path.to_string_lossy().contains("Library/Caches"));

          #[cfg(target_os = "windows")]
          {
            if let Ok(localappdata) = env::var("LOCALAPPDATA") {
              assert_eq!(cache_path, PathBuf::from(localappdata));
            }
          }
        }
      });
    }
  }

  mod config_home {
    use temp_env::{with_var, with_var_unset};

    use super::*;

    #[test]
    fn respects_xdg_config_home() {
      let test_path = if cfg!(windows) { "C:\\test\\config" } else { "/test/config" };
      with_var("XDG_CONFIG_HOME", Some(test_path), || {
        let result = Dir::config_home();
        assert_eq!(result, Some(PathBuf::from(test_path)));
      });
    }

    #[test]
    fn ignores_relative_xdg_config_home() {
      with_var("XDG_CONFIG_HOME", Some("relative/config"), || {
        let result = Dir::config_home();
        if let Some(path) = result {
          assert!(path.is_absolute());
        }
      });
    }

    #[test]
    fn uses_platform_default_when_xdg_unset() {
      with_var_unset("XDG_CONFIG_HOME", || {
        let result = Dir::config_home();
        if let Some(config_path) = result {
          assert!(config_path.is_absolute());

          #[cfg(target_os = "linux")]
          assert!(config_path.to_string_lossy().ends_with(".config"));

          #[cfg(target_os = "macos")]
          assert!(config_path.to_string_lossy().contains("Library/Application Support"));

          #[cfg(target_os = "windows")]
          {
            if let Ok(appdata) = env::var("APPDATA") {
              assert_eq!(config_path, PathBuf::from(appdata));
            }
          }
        }
      });
    }
  }

  mod config_local {
    use super::*;

    #[test]
    fn uses_localappdata_on_windows() {
      let result = Dir::config_local();
      if let Some(config_local_path) = result {
        assert!(config_local_path.is_absolute());

        #[cfg(target_os = "windows")]
        {
          if let Ok(localappdata) = env::var("LOCALAPPDATA") {
            assert_eq!(config_local_path, PathBuf::from(localappdata));
          }
        }

        #[cfg(not(target_os = "windows"))]
        {
          assert_eq!(Some(config_local_path), Dir::config_home());
        }
      }
    }

    #[test]
    fn matches_config_home_on_non_windows() {
      #[cfg(not(target_os = "windows"))]
      {
        let config_local = Dir::config_local();
        let config_home = Dir::config_home();
        assert_eq!(config_local, config_home);
      }
    }
  }

  mod data_home {
    use temp_env::{with_var, with_var_unset};

    use super::*;

    #[test]
    fn respects_xdg_data_home() {
      let test_path = if cfg!(windows) { "C:\\test\\data" } else { "/test/data" };
      with_var("XDG_DATA_HOME", Some(test_path), || {
        let result = Dir::data_home();
        assert_eq!(result, Some(PathBuf::from(test_path)));
      });
    }

    #[test]
    fn ignores_relative_xdg_data_home() {
      with_var("XDG_DATA_HOME", Some("relative/data"), || {
        let result = Dir::data_home();
        if let Some(path) = result {
          assert!(path.is_absolute());
        }
      });
    }

    #[test]
    fn uses_platform_default_when_xdg_unset() {
      with_var_unset("XDG_DATA_HOME", || {
        let result = Dir::data_home();
        if let Some(data_path) = result {
          assert!(data_path.is_absolute());

          #[cfg(target_os = "linux")]
          assert!(data_path.to_string_lossy().ends_with(".local/share"));

          #[cfg(target_os = "macos")]
          assert!(data_path.to_string_lossy().contains("Library/Application Support"));

          #[cfg(target_os = "windows")]
          {
            if let Ok(appdata) = env::var("APPDATA") {
              assert_eq!(data_path, PathBuf::from(appdata));
            }
          }
        }
      });
    }
  }

  mod data_local {
    use super::*;

    #[test]
    fn uses_localappdata_on_windows() {
      let result = Dir::data_local();
      if let Some(data_local_path) = result {
        assert!(data_local_path.is_absolute());

        #[cfg(target_os = "windows")]
        {
          if let Ok(localappdata) = env::var("LOCALAPPDATA") {
            assert_eq!(data_local_path, PathBuf::from(localappdata));
          }
        }

        #[cfg(not(target_os = "windows"))]
        {
          assert_eq!(Some(data_local_path), Dir::data_home());
        }
      }
    }

    #[test]
    fn matches_data_home_on_non_windows() {
      #[cfg(not(target_os = "windows"))]
      {
        let data_local = Dir::data_local();
        let data_home = Dir::data_home();
        assert_eq!(data_local, data_home);
      }
    }
  }

  mod desktop {
    use temp_env::{with_var, with_var_unset};

    use super::*;

    #[test]
    fn respects_xdg_desktop_dir() {
      let test_path = if cfg!(windows) { "C:\\test\\desktop" } else { "/test/desktop" };
      with_var("XDG_DESKTOP_DIR", Some(test_path), || {
        let result = Dir::desktop();
        assert_eq!(result, Some(PathBuf::from(test_path)));
      });
    }

    #[test]
    fn ignores_relative_xdg_desktop_dir() {
      with_var("XDG_DESKTOP_DIR", Some("relative/desktop"), || {
        let result = Dir::desktop();
        if let Some(path) = result {
          assert!(path.is_absolute());
        }
      });
    }

    #[test]
    fn uses_platform_default_when_xdg_unset() {
      with_var_unset("XDG_DESKTOP_DIR", || {
        let result = Dir::desktop();
        if let Some(desktop_path) = result {
          assert!(desktop_path.is_absolute());
          assert!(desktop_path.to_string_lossy().ends_with("Desktop"));
        }
      });
    }
  }

  mod documents {
    use temp_env::{with_var, with_var_unset};

    use super::*;

    #[test]
    fn respects_xdg_documents_dir() {
      let test_path = if cfg!(windows) { "C:\\test\\documents" } else { "/test/documents" };
      with_var("XDG_DOCUMENTS_DIR", Some(test_path), || {
        let result = Dir::documents();
        assert_eq!(result, Some(PathBuf::from(test_path)));
      });
    }

    #[test]
    fn ignores_relative_xdg_documents_dir() {
      with_var("XDG_DOCUMENTS_DIR", Some("relative/documents"), || {
        let result = Dir::documents();
        if let Some(path) = result {
          assert!(path.is_absolute());
        }
      });
    }

    #[test]
    fn uses_platform_default_when_xdg_unset() {
      with_var_unset("XDG_DOCUMENTS_DIR", || {
        let result = Dir::documents();
        if let Some(documents_path) = result {
          assert!(documents_path.is_absolute());
          assert!(documents_path.to_string_lossy().ends_with("Documents"));
        }
      });
    }
  }

  mod downloads {
    use temp_env::{with_var, with_var_unset};

    use super::*;

    #[test]
    fn respects_xdg_download_dir() {
      let test_path = if cfg!(windows) { "C:\\test\\downloads" } else { "/test/downloads" };
      with_var("XDG_DOWNLOAD_DIR", Some(test_path), || {
        let result = Dir::downloads();
        assert_eq!(result, Some(PathBuf::from(test_path)));
      });
    }

    #[test]
    fn ignores_relative_xdg_download_dir() {
      with_var("XDG_DOWNLOAD_DIR", Some("relative/downloads"), || {
        let result = Dir::downloads();
        if let Some(path) = result {
          assert!(path.is_absolute());
        }
      });
    }

    #[test]
    fn uses_platform_default_when_xdg_unset() {
      with_var_unset("XDG_DOWNLOAD_DIR", || {
        let result = Dir::downloads();
        if let Some(downloads_path) = result {
          assert!(downloads_path.is_absolute());
          assert!(downloads_path.to_string_lossy().ends_with("Downloads"));
        }
      });
    }
  }

  mod fonts {
    use super::*;

    #[test]
    fn returns_platform_specific_path() {
      let result = Dir::fonts();

      #[cfg(target_os = "linux")]
      if let Some(fonts_path) = result {
        assert!(fonts_path.is_absolute());
        assert!(fonts_path.to_string_lossy().ends_with(".local/share/fonts"));
      }

      #[cfg(target_os = "macos")]
      if let Some(fonts_path) = result {
        assert!(fonts_path.is_absolute());
        assert!(fonts_path.to_string_lossy().ends_with("Library/Fonts"));
      }

      #[cfg(target_os = "windows")]
      assert_eq!(result, None);
    }

    #[test]
    fn returns_none_on_windows() {
      #[cfg(target_os = "windows")]
      {
        let result = Dir::fonts();
        assert_eq!(result, None);
      }
    }

    #[test]
    fn returns_some_on_unix() {
      #[cfg(any(target_os = "linux", target_os = "macos"))]
      {
        let result = Dir::fonts();
        assert!(result.is_some());
        if let Some(path) = result {
          assert!(path.is_absolute());
        }
      }
    }
  }

  mod home {
    use super::*;

    #[test]
    fn returns_absolute_path_when_available() {
      let result = Dir::home();
      if let Some(home_path) = result {
        assert!(home_path.is_absolute());
      }
    }

    #[test]
    fn delegates_to_std_env_home_dir() {
      let std_result = std::env::home_dir();
      let our_result = Dir::home();
      assert_eq!(std_result, our_result);
    }
  }

  mod music {
    use temp_env::{with_var, with_var_unset};

    use super::*;

    #[test]
    fn respects_xdg_music_dir() {
      let test_path = if cfg!(windows) { "C:\\test\\music" } else { "/test/music" };
      with_var("XDG_MUSIC_DIR", Some(test_path), || {
        let result = Dir::music();
        assert_eq!(result, Some(PathBuf::from(test_path)));
      });
    }

    #[test]
    fn ignores_relative_xdg_music_dir() {
      with_var("XDG_MUSIC_DIR", Some("relative/music"), || {
        let result = Dir::music();
        if let Some(path) = result {
          assert!(path.is_absolute());
        }
      });
    }

    #[test]
    fn uses_platform_default_when_xdg_unset() {
      with_var_unset("XDG_MUSIC_DIR", || {
        let result = Dir::music();
        if let Some(music_path) = result {
          assert!(music_path.is_absolute());
          assert!(music_path.to_string_lossy().ends_with("Music"));
        }
      });
    }
  }

  mod pictures {
    use temp_env::{with_var, with_var_unset};

    use super::*;

    #[test]
    fn respects_xdg_pictures_dir() {
      let test_path = if cfg!(windows) { "C:\\test\\pictures" } else { "/test/pictures" };
      with_var("XDG_PICTURES_DIR", Some(test_path), || {
        let result = Dir::pictures();
        assert_eq!(result, Some(PathBuf::from(test_path)));
      });
    }

    #[test]
    fn ignores_relative_xdg_pictures_dir() {
      with_var("XDG_PICTURES_DIR", Some("relative/pictures"), || {
        let result = Dir::pictures();
        if let Some(path) = result {
          assert!(path.is_absolute());
        }
      });
    }

    #[test]
    fn uses_platform_default_when_xdg_unset() {
      with_var_unset("XDG_PICTURES_DIR", || {
        let result = Dir::pictures();
        if let Some(pictures_path) = result {
          assert!(pictures_path.is_absolute());
          assert!(pictures_path.to_string_lossy().ends_with("Pictures"));
        }
      });
    }
  }

  mod preferences {
    use super::*;

    #[test]
    fn returns_platform_specific_path() {
      let result = Dir::preferences();
      if let Some(preferences_path) = result {
        assert!(preferences_path.is_absolute());

        #[cfg(target_os = "macos")]
        assert!(preferences_path.to_string_lossy().ends_with("Library/Preferences"));

        #[cfg(not(target_os = "macos"))]
        assert_eq!(Some(preferences_path), Dir::config_home());
      }
    }

    #[test]
    fn matches_config_home_on_non_macos() {
      #[cfg(not(target_os = "macos"))]
      {
        let preferences = Dir::preferences();
        let config_home = Dir::config_home();
        assert_eq!(preferences, config_home);
      }
    }

    #[test]
    fn uses_library_preferences_on_macos() {
      #[cfg(target_os = "macos")]
      {
        let result = Dir::preferences();
        if let Some(path) = result {
          assert!(path.to_string_lossy().ends_with("Library/Preferences"));
        }
      }
    }
  }

  mod publicshare {
    use temp_env::{with_var, with_var_unset};

    use super::*;

    #[test]
    fn respects_xdg_publicshare_dir() {
      let test_path = if cfg!(windows) { "C:\\test\\public" } else { "/test/public" };
      with_var("XDG_PUBLICSHARE_DIR", Some(test_path), || {
        let result = Dir::publicshare();
        assert_eq!(result, Some(PathBuf::from(test_path)));
      });
    }

    #[test]
    fn ignores_relative_xdg_publicshare_dir() {
      with_var("XDG_PUBLICSHARE_DIR", Some("relative/public"), || {
        let result = Dir::publicshare();
        if let Some(path) = result {
          assert!(path.is_absolute());
        }
      });
    }

    #[test]
    fn uses_platform_default_when_xdg_unset() {
      with_var_unset("XDG_PUBLICSHARE_DIR", || {
        let result = Dir::publicshare();
        if let Some(public_path) = result {
          assert!(public_path.is_absolute());

          #[cfg(target_os = "windows")]
          assert_eq!(public_path, PathBuf::from("C:\\Users\\Public"));

          #[cfg(any(target_os = "linux", target_os = "macos"))]
          assert!(public_path.to_string_lossy().ends_with("Public"));
        }
      });
    }

    #[test]
    fn uses_system_public_on_windows() {
      #[cfg(target_os = "windows")]
      {
        with_var_unset("XDG_PUBLICSHARE_DIR", || {
          let result = Dir::publicshare();
          assert_eq!(result, Some(PathBuf::from("C:\\Users\\Public")));
        });
      }
    }
  }

  mod runtime {
    use temp_env::{with_var, with_var_unset};

    use super::*;

    #[test]
    fn respects_xdg_runtime_dir() {
      let test_path = if cfg!(windows) { "C:\\test\\runtime" } else { "/test/runtime" };
      with_var("XDG_RUNTIME_DIR", Some(test_path), || {
        let result = Dir::runtime();
        assert_eq!(result, Some(PathBuf::from(test_path)));
      });
    }

    #[test]
    fn ignores_relative_xdg_runtime_dir() {
      with_var("XDG_RUNTIME_DIR", Some("relative/runtime"), || {
        let result = Dir::runtime();
        if let Some(path) = result {
          assert!(path.is_absolute());
        }
      });
    }

    #[test]
    fn uses_platform_default_when_xdg_unset() {
      with_var_unset("XDG_RUNTIME_DIR", || {
        let result = Dir::runtime();
        if let Some(runtime_path) = result {
          assert!(runtime_path.is_absolute());

          #[cfg(any(target_os = "linux", target_os = "macos"))]
          {
            let path_str = runtime_path.to_string_lossy();
            assert!(path_str.contains("tmp") || path_str.starts_with("/var/folders"));
          }

          #[cfg(target_os = "windows")]
          {
            if let Ok(temp) = env::var("TEMP") {
              assert_eq!(runtime_path, PathBuf::from(temp));
            }
          }
        }
      });
    }

    #[test]
    fn falls_back_to_tmp_on_unix() {
      #[cfg(any(target_os = "linux", target_os = "macos"))]
      {
        with_var_unset("XDG_RUNTIME_DIR", || {
          with_var_unset("TMPDIR", || {
            let result = Dir::runtime();
            assert_eq!(result, Some(PathBuf::from("/tmp")));
          });
        });
      }
    }
  }

  mod state_home {
    use temp_env::{with_var, with_var_unset};

    use super::*;

    #[test]
    fn respects_xdg_state_home() {
      let test_path = if cfg!(windows) { "C:\\test\\state" } else { "/test/state" };
      with_var("XDG_STATE_HOME", Some(test_path), || {
        let result = Dir::state_home();
        assert_eq!(result, Some(PathBuf::from(test_path)));
      });
    }

    #[test]
    fn ignores_relative_xdg_state_home() {
      with_var("XDG_STATE_HOME", Some("relative/state"), || {
        let result = Dir::state_home();
        if let Some(path) = result {
          assert!(path.is_absolute());
        }
      });
    }

    #[test]
    fn uses_platform_default_when_xdg_unset() {
      with_var_unset("XDG_STATE_HOME", || {
        let result = Dir::state_home();
        if let Some(state_path) = result {
          assert!(state_path.is_absolute());

          #[cfg(target_os = "linux")]
          assert!(state_path.to_string_lossy().ends_with(".local/state"));

          #[cfg(target_os = "macos")]
          assert!(state_path.to_string_lossy().contains("Library/Application Support"));

          #[cfg(target_os = "windows")]
          {
            if let Ok(localappdata) = env::var("LOCALAPPDATA") {
              assert_eq!(state_path, PathBuf::from(localappdata));
            }
          }
        }
      });
    }
  }

  mod templates {
    use temp_env::{with_var, with_var_unset};

    use super::*;

    #[test]
    fn respects_xdg_templates_dir() {
      let test_path = if cfg!(windows) { "C:\\test\\templates" } else { "/test/templates" };
      with_var("XDG_TEMPLATES_DIR", Some(test_path), || {
        let result = Dir::templates();
        assert_eq!(result, Some(PathBuf::from(test_path)));
      });
    }

    #[test]
    fn ignores_relative_xdg_templates_dir() {
      with_var("XDG_TEMPLATES_DIR", Some("relative/templates"), || {
        let result = Dir::templates();
        if let Some(path) = result {
          assert!(path.is_absolute());
        }
      });
    }

    #[test]
    fn uses_platform_default_when_xdg_unset() {
      with_var_unset("XDG_TEMPLATES_DIR", || {
        let result = Dir::templates();
        if let Some(templates_path) = result {
          assert!(templates_path.is_absolute());
          assert!(templates_path.to_string_lossy().ends_with("Templates"));
        }
      });
    }
  }

  mod videos {
    use temp_env::{with_var, with_var_unset};

    use super::*;

    #[test]
    fn respects_xdg_videos_dir() {
      let test_path = if cfg!(windows) { "C:\\test\\videos" } else { "/test/videos" };
      with_var("XDG_VIDEOS_DIR", Some(test_path), || {
        let result = Dir::videos();
        assert_eq!(result, Some(PathBuf::from(test_path)));
      });
    }

    #[test]
    fn ignores_relative_xdg_videos_dir() {
      with_var("XDG_VIDEOS_DIR", Some("relative/videos"), || {
        let result = Dir::videos();
        if let Some(path) = result {
          assert!(path.is_absolute());
        }
      });
    }

    #[test]
    fn uses_platform_default_when_xdg_unset() {
      with_var_unset("XDG_VIDEOS_DIR", || {
        let result = Dir::videos();
        if let Some(videos_path) = result {
          assert!(videos_path.is_absolute());

          #[cfg(target_os = "linux")]
          assert!(videos_path.to_string_lossy().ends_with("Videos"));

          #[cfg(target_os = "macos")]
          assert!(videos_path.to_string_lossy().ends_with("Movies"));

          #[cfg(target_os = "windows")]
          assert!(videos_path.to_string_lossy().ends_with("Videos"));
        }
      });
    }
  }
}
