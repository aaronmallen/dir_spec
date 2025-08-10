use std::{env, path::PathBuf};

use eyre::Result;

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
/// let config = Dir::config_home()?;
/// println!("Config directory: {}", config.display());
///
/// // Get cache directory (respects XDG_CACHE_HOME if set)
/// let cache = Dir::cache_home()?;
/// println!("Cache directory: {}", cache.display());
///
/// // Get videos directory (new cleaner API)
/// let videos = Dir::videos()?;
/// println!("Videos directory: {}", videos.display());
/// # Ok::<(), eyre::Error>(())
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
  /// # fn main() -> Result<(), eyre::Error> {
  /// let bin_dir = Dir::bin_home()?;
  /// # Ok(())
  /// # }
  /// ```
  pub fn bin_home() -> Result<PathBuf> {
    if let Some(path) = Self::resolve_xdg_path("XDG_BIN_HOME") {
      return Ok(path);
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    {
      Ok(Self::home()?.join(".local/bin"))
    }

    #[cfg(target_os = "windows")]
    {
      env::var("LOCALAPPDATA")
        .map(|path| PathBuf::from(path).join("Programs"))
        .map_err(|_| eyre::eyre!("Failed to resolve bin directory"))
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
  /// # fn main() -> Result<(), eyre::Error> {
  /// let cache_dir = Dir::cache_home()?;
  /// # Ok(())
  /// # }
  /// ```
  pub fn cache_home() -> Result<PathBuf> {
    Self::resolve_xdg_path("XDG_CACHE_HOME")
      .map_or_else(|| Self::get_platform_default("Library/Caches", "LOCALAPPDATA", ".cache"), Ok)
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
  /// # fn main() -> Result<(), eyre::Error> {
  /// let config_dir = Dir::config_home()?;
  /// # Ok(())
  /// # }
  /// ```
  pub fn config_home() -> Result<PathBuf> {
    Self::resolve_xdg_path("XDG_CONFIG_HOME")
      .map_or_else(|| Self::get_platform_default("Library/Application Support", "APPDATA", ".config"), Ok)
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
  /// # fn main() -> Result<(), eyre::Error> {
  /// let data_dir = Dir::data_home()?;
  /// # Ok(())
  /// # }
  /// ```
  pub fn data_home() -> Result<PathBuf> {
    Self::resolve_xdg_path("XDG_DATA_HOME")
      .map_or_else(|| Self::get_platform_default("Library/Application Support", "APPDATA", ".local/share"), Ok)
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
  /// # fn main() -> Result<(), eyre::Error> {
  /// let desktop = Dir::desktop()?;
  /// # Ok(())
  /// # }
  /// ```
  pub fn desktop() -> Result<PathBuf> {
    Self::resolve_xdg_path("XDG_DESKTOP_DIR").map_or_else(
      || Self::get_platform_default_with_windows_subdir("Desktop", "USERPROFILE", "Desktop", "Desktop"),
      Ok,
    )
  }

  /// Returns the user's desktop directory.
  ///
  /// **Deprecated**: Use [`Dir::desktop()`] instead.
  ///
  /// [`Dir::desktop()`]: Self::desktop
  #[deprecated(since = "0.1.0", note = "Use `Dir::desktop()` instead")]
  pub fn desktop_dir() -> Result<PathBuf> {
    Self::desktop()
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
  /// # fn main() -> Result<(), eyre::Error> {
  /// let documents = Dir::documents()?;
  /// # Ok(())
  /// # }
  /// ```
  pub fn documents() -> Result<PathBuf> {
    Self::resolve_xdg_path("XDG_DOCUMENTS_DIR").map_or_else(
      || Self::get_platform_default_with_windows_subdir("Documents", "USERPROFILE", "Documents", "Documents"),
      Ok,
    )
  }

  /// Returns the user's documents directory.
  ///
  /// **Deprecated**: Use [`Dir::documents()`] instead.
  ///
  /// [`Dir::documents()`]: Self::documents
  #[deprecated(since = "0.1.0", note = "Use `Dir::documents()` instead")]
  pub fn documents_dir() -> Result<PathBuf> {
    Self::documents()
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
  /// # fn main() -> Result<(), eyre::Error> {
  /// let downloads = Dir::downloads()?;
  /// # Ok(())
  /// # }
  /// ```
  pub fn downloads() -> Result<PathBuf> {
    Self::resolve_xdg_path("XDG_DOWNLOAD_DIR").map_or_else(
      || Self::get_platform_default_with_windows_subdir("Downloads", "USERPROFILE", "Downloads", "Downloads"),
      Ok,
    )
  }

  /// Returns the user's downloads directory.
  ///
  /// **Deprecated**: Use [`Dir::downloads()`] instead.
  ///
  /// [`Dir::downloads()`]: Self::downloads
  #[deprecated(since = "0.1.0", note = "Use `Dir::downloads()` instead")]
  pub fn download_dir() -> Result<PathBuf> {
    Self::downloads()
  }

  /// Returns the user's home directory.
  ///
  /// Uses platform-specific methods to resolve the home directory:
  /// - **Unix**: `$HOME` environment variable, falling back to passwd database lookup
  /// - **Windows**: `%USERPROFILE%`, falling back to `%HOMEDRIVE%%HOMEPATH%`
  ///
  /// # Examples
  ///
  /// ```rust
  /// use dir_spec::Dir;
  /// # fn main() -> Result<(), eyre::Error> {
  /// let home_dir = Dir::home()?;
  /// # Ok(())
  /// # }
  /// ```
  pub fn home() -> Result<PathBuf> {
    Ok(std::env::home_dir().unwrap())
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
  /// # fn main() -> Result<(), eyre::Error> {
  /// let music = Dir::music()?;
  /// # Ok(())
  /// # }
  /// ```
  pub fn music() -> Result<PathBuf> {
    Self::resolve_xdg_path("XDG_MUSIC_DIR")
      .map_or_else(|| Self::get_platform_default_with_windows_subdir("Music", "USERPROFILE", "Music", "Music"), Ok)
  }

  /// Returns the user's music directory.
  ///
  /// **Deprecated**: Use [`Dir::music()`] instead.
  ///
  /// [`Dir::music()`]: Self::music
  #[deprecated(since = "0.1.0", note = "Use `Dir::music()` instead")]
  pub fn music_dir() -> Result<PathBuf> {
    Self::music()
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
  /// # fn main() -> Result<(), eyre::Error> {
  /// let pictures = Dir::pictures()?;
  /// # Ok(())
  /// # }
  /// ```
  pub fn pictures() -> Result<PathBuf> {
    Self::resolve_xdg_path("XDG_PICTURES_DIR").map_or_else(
      || Self::get_platform_default_with_windows_subdir("Pictures", "USERPROFILE", "Pictures", "Pictures"),
      Ok,
    )
  }

  /// Returns the user's pictures directory.
  ///
  /// **Deprecated**: Use [`Dir::pictures()`] instead.
  ///
  /// [`Dir::pictures()`]: Self::pictures
  #[deprecated(since = "0.1.0", note = "Use `Dir::pictures()` instead")]
  pub fn pictures_dir() -> Result<PathBuf> {
    Self::pictures()
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
  /// # fn main() -> Result<(), eyre::Error> {
  /// let public = Dir::publicshare()?;
  /// # Ok(())
  /// # }
  /// ```
  pub fn publicshare() -> Result<PathBuf> {
    if let Some(path) = Self::resolve_xdg_path("XDG_PUBLICSHARE_DIR") {
      return Ok(path);
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

  /// Returns the user's public share directory.
  ///
  /// **Deprecated**: Use [`Dir::publicshare()`] instead.
  ///
  /// [`Dir::publicshare()`]: Self::publicshare
  #[deprecated(since = "0.1.0", note = "Use `Dir::publicshare()` instead")]
  pub fn publicshare_dir() -> Result<PathBuf> {
    Self::publicshare()
  }

  /// Returns the user's runtime directory.
  ///
  /// Checks `XDG_RUNTIME_DIR` first, then falls back to platform defaults:
  /// - **Linux**: `/run/user/{uid}`
  /// - **macOS**: `$TMPDIR` or `/tmp`
  /// - **Windows**: `%TEMP%`
  ///
  /// # Examples
  ///
  /// ```rust
  /// use dir_spec::Dir;
  /// # fn main() -> Result<(), eyre::Error> {
  /// let runtime = Dir::runtime()?;
  /// # Ok(())
  /// # }
  /// ```
  pub fn runtime() -> Result<PathBuf> {
    if let Some(path) = Self::resolve_xdg_path("XDG_RUNTIME_DIR") {
      return Ok(path);
    }

    #[cfg(target_os = "macos")]
    {
      if let Ok(tmpdir) = env::var("TMPDIR") { Ok(PathBuf::from(tmpdir)) } else { Ok(PathBuf::from("/tmp")) }
    }

    #[cfg(target_os = "windows")]
    {
      env::var("TEMP").map(PathBuf::from).map_err(|_| eyre::eyre!("Failed to resolve runtime directory"))
    }

    #[cfg(target_os = "linux")]
    {
      #[allow(unsafe_code)]
      let uid = unsafe { libc::getuid() };
      Ok(PathBuf::from(format!("/run/user/{}", uid)))
    }
  }

  /// Returns the user's runtime directory.
  ///
  /// **Deprecated**: Use [`Dir::runtime()`] instead.
  ///
  /// [`Dir::runtime()`]: Self::runtime
  #[deprecated(since = "0.1.0", note = "Use `Dir::runtime()` instead")]
  pub fn runtime_dir() -> Result<PathBuf> {
    Self::runtime()
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
  /// # fn main() -> Result<(), eyre::Error> {
  /// let state_dir = Dir::state_home()?;
  /// # Ok(())
  /// # }
  /// ```
  pub fn state_home() -> Result<PathBuf> {
    Self::resolve_xdg_path("XDG_STATE_HOME")
      .map_or_else(|| Self::get_platform_default("Library/Application Support", "LOCALAPPDATA", ".local/state"), Ok)
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
  /// # fn main() -> Result<(), eyre::Error> {
  /// let templates = Dir::templates()?;
  /// # Ok(())
  /// # }
  /// ```
  pub fn templates() -> Result<PathBuf> {
    Self::resolve_xdg_path("XDG_TEMPLATES_DIR").map_or_else(
      || Self::get_platform_default_with_windows_subdir("Templates", "USERPROFILE", "Templates", "Templates"),
      Ok,
    )
  }

  /// Returns the user's templates directory.
  ///
  /// **Deprecated**: Use [`Dir::templates()`] instead.
  ///
  /// [`Dir::templates()`]: Self::templates
  #[deprecated(since = "0.1.0", note = "Use `Dir::templates()` instead")]
  pub fn templates_dir() -> Result<PathBuf> {
    Self::templates()
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
  /// # fn main() -> Result<(), eyre::Error> {
  /// let videos = Dir::videos()?;
  /// # Ok(())
  /// # }
  /// ```
  pub fn videos() -> Result<PathBuf> {
    Self::resolve_xdg_path("XDG_VIDEOS_DIR")
      .map_or_else(|| Self::get_platform_default_with_windows_subdir("Movies", "USERPROFILE", "Videos", "Videos"), Ok)
  }

  /// Returns the user's videos directory.
  ///
  /// **Deprecated**: Use [`Dir::videos()`] instead.
  ///
  /// [`Dir::videos()`]: Self::videos
  #[deprecated(since = "0.1.0", note = "Use `Dir::videos()` instead")]
  pub fn videos_dir() -> Result<PathBuf> {
    Self::videos()
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
  ) -> Result<PathBuf> {
    #[cfg(target_os = "macos")]
    {
      Ok(Self::home()?.join(macos_path))
    }

    #[cfg(target_os = "windows")]
    {
      env::var(windows_env).map(PathBuf::from).map_err(|_| eyre::eyre!("Failed to resolve directory"))
    }

    #[cfg(target_os = "linux")]
    {
      Ok(Self::home()?.join(linux_path))
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
  ) -> Result<PathBuf> {
    #[cfg(target_os = "macos")]
    {
      Ok(Self::home()?.join(macos_path))
    }

    #[cfg(target_os = "windows")]
    {
      env::var(windows_env)
        .map(|path| PathBuf::from(path).join(windows_subdir))
        .map_err(|_| eyre::eyre!("Failed to resolve directory"))
    }

    #[cfg(target_os = "linux")]
    {
      Ok(Self::home()?.join(linux_path))
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
  use std::env;

  use super::*;

  #[test]
  #[allow(unsafe_code)]
  fn test_resolve_xdg_path_absolute() {
    unsafe {
      env::set_var("TEST_XDG_VAR", "/absolute/path");
    }
    let result = Dir::resolve_xdg_path("TEST_XDG_VAR");
    assert_eq!(result, Some(PathBuf::from("/absolute/path")));
    unsafe {
      env::remove_var("TEST_XDG_VAR");
    }
  }

  #[test]
  #[allow(unsafe_code)]
  fn test_resolve_xdg_path_relative_ignored() {
    unsafe {
      env::set_var("TEST_XDG_VAR", "relative/path");
    }
    let result = Dir::resolve_xdg_path("TEST_XDG_VAR");
    assert_eq!(result, None);
    unsafe {
      env::remove_var("TEST_XDG_VAR");
    }
  }

  #[test]
  #[allow(unsafe_code)]
  fn test_resolve_xdg_path_unset() {
    unsafe {
      env::remove_var("TEST_XDG_VAR");
    }
    let result = Dir::resolve_xdg_path("TEST_XDG_VAR");
    assert_eq!(result, None);
  }

  #[test]
  fn test_home_directory() {
    let home = Dir::home();
    assert!(home.is_ok());
    let home_path = home.unwrap();
    assert!(home_path.is_absolute());
  }

  #[test]
  #[allow(unsafe_code)]
  fn test_config_home_default() {
    unsafe {
      env::remove_var("XDG_CONFIG_HOME");
    }
    let config = Dir::config_home();
    assert!(config.is_ok());
    let config_path = config.unwrap();
    assert!(config_path.is_absolute());

    #[cfg(target_os = "linux")]
    assert!(config_path.to_string_lossy().ends_with(".config"));

    #[cfg(target_os = "macos")]
    assert!(config_path.to_string_lossy().contains("Library/Application Support"));
  }

  #[test]
  #[allow(unsafe_code)]
  fn test_config_home_xdg_override() {
    let test_path = if cfg!(windows) { "C:\\test\\config" } else { "/test/config" };
    unsafe {
      env::set_var("XDG_CONFIG_HOME", test_path);
    }
    let config = Dir::config_home();
    assert!(config.is_ok());
    assert_eq!(config.unwrap(), PathBuf::from(test_path));
    unsafe {
      env::remove_var("XDG_CONFIG_HOME");
    }
  }

  #[test]
  #[allow(unsafe_code)]
  fn test_cache_home_default() {
    unsafe {
      env::remove_var("XDG_CACHE_HOME");
    }
    let cache = Dir::cache_home();
    assert!(cache.is_ok());
    let cache_path = cache.unwrap();
    assert!(cache_path.is_absolute());

    #[cfg(target_os = "linux")]
    assert!(cache_path.to_string_lossy().ends_with(".cache"));

    #[cfg(target_os = "macos")]
    assert!(cache_path.to_string_lossy().contains("Library/Caches"));
  }

  #[test]
  #[allow(unsafe_code)]
  fn test_data_home_default() {
    unsafe {
      env::remove_var("XDG_DATA_HOME");
    }
    let data = Dir::data_home();
    assert!(data.is_ok());
    let data_path = data.unwrap();
    assert!(data_path.is_absolute());

    #[cfg(target_os = "linux")]
    assert!(data_path.to_string_lossy().ends_with(".local/share"));

    #[cfg(target_os = "macos")]
    assert!(data_path.to_string_lossy().contains("Library/Application Support"));
  }

  #[test]
  #[allow(unsafe_code)]
  fn test_bin_home_default() {
    unsafe {
      env::remove_var("XDG_BIN_HOME");
    }
    let bin = Dir::bin_home();
    assert!(bin.is_ok());
    let bin_path = bin.unwrap();
    assert!(bin_path.is_absolute());

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    assert!(bin_path.to_string_lossy().ends_with(".local/bin"));

    #[cfg(target_os = "windows")]
    assert!(bin_path.to_string_lossy().contains("Programs"));
  }

  #[test]
  #[allow(unsafe_code)]
  fn test_runtime_default() {
    unsafe {
      env::remove_var("XDG_RUNTIME_DIR");
    }
    let runtime = Dir::runtime();
    assert!(runtime.is_ok());
    let runtime_path = runtime.unwrap();
    assert!(runtime_path.is_absolute());

    #[cfg(target_os = "linux")]
    assert!(runtime_path.to_string_lossy().starts_with("/run/user/"));

    #[cfg(target_os = "macos")]
    {
      let path_str = runtime_path.to_string_lossy();
      assert!(path_str.contains("tmp") || path_str.starts_with("/var/folders"));
    }
  }

  #[test]
  #[allow(unsafe_code)]
  fn test_desktop_default() {
    unsafe {
      env::remove_var("XDG_DESKTOP_DIR");
    }
    let desktop = Dir::desktop();
    assert!(desktop.is_ok());
    let desktop_path = desktop.unwrap();
    assert!(desktop_path.is_absolute());
    assert!(desktop_path.to_string_lossy().ends_with("Desktop"));
  }

  #[test]
  #[allow(unsafe_code)]
  fn test_videos_platform_differences() {
    unsafe {
      env::remove_var("XDG_VIDEOS_DIR");
    }
    let videos = Dir::videos();
    assert!(videos.is_ok());
    let videos_path = videos.unwrap();
    assert!(videos_path.is_absolute());

    #[cfg(target_os = "linux")]
    assert!(videos_path.to_string_lossy().ends_with("Videos"));

    #[cfg(target_os = "macos")]
    assert!(videos_path.to_string_lossy().ends_with("Movies"));

    #[cfg(target_os = "windows")]
    assert!(videos_path.to_string_lossy().ends_with("Videos"));
  }

  #[test]
  #[allow(unsafe_code)]
  fn test_publicshare_windows_absolute() {
    unsafe {
      env::remove_var("XDG_PUBLICSHARE_DIR");
    }
    let public = Dir::publicshare();
    assert!(public.is_ok());
    let public_path = public.unwrap();
    assert!(public_path.is_absolute());

    #[cfg(target_os = "windows")]
    assert_eq!(public_path, PathBuf::from("C:\\Users\\Public"));

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    assert!(public_path.to_string_lossy().ends_with("Public"));
  }

  #[test]
  #[allow(unsafe_code)]
  fn test_deprecated_methods_work() {
    #[allow(deprecated)]
    {
      assert!(Dir::desktop_dir().is_ok());
      assert!(Dir::documents_dir().is_ok());
      assert!(Dir::download_dir().is_ok());
      assert!(Dir::music_dir().is_ok());
      assert!(Dir::pictures_dir().is_ok());
      assert!(Dir::publicshare_dir().is_ok());
      assert!(Dir::runtime_dir().is_ok());
      assert!(Dir::templates_dir().is_ok());
      assert!(Dir::videos_dir().is_ok());
    }
  }

  #[test]
  #[allow(unsafe_code)]
  fn test_new_methods_equivalent_to_deprecated() {
    unsafe {
      env::remove_var("XDG_DESKTOP_DIR");
      env::remove_var("XDG_DOCUMENTS_DIR");
      env::remove_var("XDG_DOWNLOAD_DIR");
      env::remove_var("XDG_MUSIC_DIR");
      env::remove_var("XDG_PICTURES_DIR");
      env::remove_var("XDG_PUBLICSHARE_DIR");
      env::remove_var("XDG_RUNTIME_DIR");
      env::remove_var("XDG_TEMPLATES_DIR");
      env::remove_var("XDG_VIDEOS_DIR");
    }

    #[allow(deprecated)]
    {
      assert_eq!(Dir::desktop().unwrap(), Dir::desktop_dir().unwrap());
      assert_eq!(Dir::documents().unwrap(), Dir::documents_dir().unwrap());
      assert_eq!(Dir::downloads().unwrap(), Dir::download_dir().unwrap());
      assert_eq!(Dir::music().unwrap(), Dir::music_dir().unwrap());
      assert_eq!(Dir::pictures().unwrap(), Dir::pictures_dir().unwrap());
      assert_eq!(Dir::publicshare().unwrap(), Dir::publicshare_dir().unwrap());
      assert_eq!(Dir::runtime().unwrap(), Dir::runtime_dir().unwrap());
      assert_eq!(Dir::templates().unwrap(), Dir::templates_dir().unwrap());
      assert_eq!(Dir::videos().unwrap(), Dir::videos_dir().unwrap());
    }
  }
}
