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
    if let Some(home_path) = home {
      assert!(home_path.is_absolute());
    }
  }

  #[test]
  #[allow(unsafe_code)]
  fn test_config_home_default() {
    unsafe {
      env::remove_var("XDG_CONFIG_HOME");
    }
    let config = Dir::config_home();
    if let Some(config_path) = config {
      assert!(config_path.is_absolute());

      #[cfg(target_os = "linux")]
      assert!(config_path.to_string_lossy().ends_with(".config"));

      #[cfg(target_os = "macos")]
      assert!(config_path.to_string_lossy().contains("Library/Application Support"));
    }
  }

  #[test]
  #[allow(unsafe_code)]
  fn test_config_home_xdg_override() {
    let test_path = if cfg!(windows) { "C:\\test\\config" } else { "/test/config" };
    unsafe {
      env::set_var("XDG_CONFIG_HOME", test_path);
    }
    let config = Dir::config_home();
    assert_eq!(config, Some(PathBuf::from(test_path)));
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
    if let Some(cache_path) = cache {
      assert!(cache_path.is_absolute());

      #[cfg(target_os = "linux")]
      assert!(cache_path.to_string_lossy().ends_with(".cache"));

      #[cfg(target_os = "macos")]
      assert!(cache_path.to_string_lossy().contains("Library/Caches"));
    }
  }

  #[test]
  #[allow(unsafe_code)]
  fn test_data_home_default() {
    unsafe {
      env::remove_var("XDG_DATA_HOME");
    }
    let data = Dir::data_home();
    if let Some(data_path) = data {
      assert!(data_path.is_absolute());

      #[cfg(target_os = "linux")]
      assert!(data_path.to_string_lossy().ends_with(".local/share"));

      #[cfg(target_os = "macos")]
      assert!(data_path.to_string_lossy().contains("Library/Application Support"));
    }
  }

  #[test]
  #[allow(unsafe_code)]
  fn test_bin_home_default() {
    unsafe {
      env::remove_var("XDG_BIN_HOME");
    }
    let bin = Dir::bin_home();
    if let Some(bin_path) = bin {
      assert!(bin_path.is_absolute());

      #[cfg(any(target_os = "linux", target_os = "macos"))]
      assert!(bin_path.to_string_lossy().ends_with(".local/bin"));

      #[cfg(target_os = "windows")]
      assert!(bin_path.to_string_lossy().contains("Programs"));
    }
  }

  #[test]
  #[allow(unsafe_code)]
  fn test_runtime_default() {
    unsafe {
      env::remove_var("XDG_RUNTIME_DIR");
    }
    let runtime = Dir::runtime();
    if let Some(runtime_path) = runtime {
      assert!(runtime_path.is_absolute());

      #[cfg(any(target_os = "linux", target_os = "macos"))]
      {
        let path_str = runtime_path.to_string_lossy();
        assert!(path_str.contains("tmp") || path_str.starts_with("/var/folders"));
      }
    }
  }

  #[test]
  #[allow(unsafe_code)]
  fn test_desktop_default() {
    unsafe {
      env::remove_var("XDG_DESKTOP_DIR");
    }
    let desktop = Dir::desktop();
    if let Some(desktop_path) = desktop {
      assert!(desktop_path.is_absolute());
      assert!(desktop_path.to_string_lossy().ends_with("Desktop"));
    }
  }

  #[test]
  #[allow(unsafe_code)]
  fn test_videos_platform_differences() {
    unsafe {
      env::remove_var("XDG_VIDEOS_DIR");
    }
    let videos = Dir::videos();
    if let Some(videos_path) = videos {
      assert!(videos_path.is_absolute());

      #[cfg(target_os = "linux")]
      assert!(videos_path.to_string_lossy().ends_with("Videos"));

      #[cfg(target_os = "macos")]
      assert!(videos_path.to_string_lossy().ends_with("Movies"));

      #[cfg(target_os = "windows")]
      assert!(videos_path.to_string_lossy().ends_with("Videos"));
    }
  }

  #[test]
  #[allow(unsafe_code)]
  fn test_publicshare_windows_absolute() {
    unsafe {
      env::remove_var("XDG_PUBLICSHARE_DIR");
    }
    let public = Dir::publicshare();
    if let Some(public_path) = public {
      assert!(public_path.is_absolute());

      #[cfg(target_os = "windows")]
      assert_eq!(public_path, PathBuf::from("C:\\Users\\Public"));

      #[cfg(any(target_os = "linux", target_os = "macos"))]
      assert!(public_path.to_string_lossy().ends_with("Public"));
    }
  }
}
