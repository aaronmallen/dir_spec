# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to [Break Versioning].

## [0.5.2] - 2026-04-11

### Added

* `xdg_bin_home_or(default)` - Returns `XDG_BIN_HOME` if set to an absolute path,
  otherwise returns the provided default
* `xdg_cache_home_or(default)` - Returns `XDG_CACHE_HOME` if set to an absolute path,
  otherwise returns the provided default
* `xdg_config_home_or(default)` - Returns `XDG_CONFIG_HOME` if set to an absolute path,
  otherwise returns the provided default
* `xdg_data_home_or(default)` - Returns `XDG_DATA_HOME` if set to an absolute path,
  otherwise returns the provided default
* `xdg_desktop_dir_or(default)` - Returns `XDG_DESKTOP_DIR` if set to an absolute path,
  otherwise returns the provided default
* `xdg_documents_dir_or(default)` - Returns `XDG_DOCUMENTS_DIR` if set to an absolute path,
  otherwise returns the provided default
* `xdg_download_dir_or(default)` - Returns `XDG_DOWNLOAD_DIR` if set to an absolute path,
  otherwise returns the provided default
* `xdg_music_dir_or(default)` - Returns `XDG_MUSIC_DIR` if set to an absolute path,
  otherwise returns the provided default
* `xdg_pictures_dir_or(default)` - Returns `XDG_PICTURES_DIR` if set to an absolute path,
  otherwise returns the provided default
* `xdg_publicshare_dir_or(default)` - Returns `XDG_PUBLICSHARE_DIR` if set to an absolute path,
  otherwise returns the provided default
* `xdg_runtime_dir_or(default)` - Returns `XDG_RUNTIME_DIR` if set to an absolute path,
  otherwise returns the provided default
* `xdg_state_home_or(default)` - Returns `XDG_STATE_HOME` if set to an absolute path,
  otherwise returns the provided default
* `xdg_templates_dir_or(default)` - Returns `XDG_TEMPLATES_DIR` if set to an absolute path,
  otherwise returns the provided default
* `xdg_videos_dir_or(default)` - Returns `XDG_VIDEOS_DIR` if set to an absolute path,
  otherwise returns the provided default

### Fixed

* All directory functions on Windows now respect their corresponding XDG environment variables before
  falling back to platform defaults

## [0.5.1] - 2026-04-10

### Fixed

* Fixed compilation on FreeBSD and other BSD-family operating systems by broadening platform support
  from `target_os = "linux"` to `target_family = "unix"` (excluding macOS)
* Renamed internal `linux` module to `unix` to reflect its broader applicability

## [0.5.0] - 2025-08-29

### Changed

* **BREAKING**: Removed `Dir` struct and moved all methods to module-level functions
* **BREAKING**: All directory resolution functions are now called directly (e.g., `config_home()` instead of
  `Dir::config_home()`)
* Updated crate-level documentation to reflect new function-based API
* Updated all examples in documentation to use new function calls

### Removed

* **BREAKING**: Removed `Dir` struct entirely

## [0.4.0] - 2025-08-28

### Changed

* Refactored internal architecture to use platform-specific modules (`linux.rs`, `macos.rs`, `windows.rs`) for improved
  code organization and maintainability
* Extracted XDG Base Directory Specification logic into dedicated `xdg` module with reusable helper functions
* Replaced inline conditional compilation scattered throughout `lib.rs` with clean module delegation pattern

## [0.3.0] - 2025-08-24

### Added

* `config_local()` - Returns user's local (non-roaming) config directory. On Windows uses `%LOCALAPPDATA%`, identical
  to `config_home()` on other platforms
* `data_local()` - Returns user's local (non-roaming) data directory. On Windows uses `%LOCALAPPDATA%`, identical to
  `data_home()` on other platforms
* `fonts()` - Returns user's fonts directory
  (`~/.local/share/fonts` on Linux, `~/Library/Fonts` on macOS, `None` on Windows)
* `preferences()` - Returns user's preferences directory. On macOS returns `~/Library/Preferences` for .plist files,
  identical to `config_home()` on other platforms

## [0.2.0] - 2025-08-10

### Changed

* **BREAKING**: All methods now return `Option<PathBuf>` instead of `Result<PathBuf>`
* **BREAKING**: `home()` method now uses `std::env::home_dir()` directly (was previously deprecated, now undeprecated)
* **BREAKING**: `runtime()` method on Linux now uses `$TMPDIR` then `/tmp` fallback instead of `/run/user/{uid}`
* Simplified error handling by removing `eyre` dependency from public API
* Updated documentation and examples to use Option pattern matching

### Removed

* **BREAKING**: Removed `eyre` dependency from public API
* **BREAKING**: Removed `libc` dependency entirely
* **BREAKING**: Removed all deprecated `*_dir()` method variants:
  * `desktop_dir()` (use `desktop()`)
  * `documents_dir()` (use `documents()`)
  * `download_dir()` (use `downloads()`)
  * `music_dir()` (use `music()`)
  * `pictures_dir()` (use `pictures()`)
  * `publicshare_dir()` (use `publicshare()`)
  * `runtime_dir()` (use `runtime()`)
  * `templates_dir()` (use `templates()`)
  * `videos_dir()` (use `videos()`)

### Fixed

* Eliminated potential panic in `home()` method by properly handling `None` case from `std::env::home_dir()`
* Removed unsafe code by eliminating libc dependency

## [0.1.0] - 2025-08-08

### Added

* New, cleaner directory resolution methods without the `_dir` suffix
  (e.g., `Dir::desktop()` replaces `Dir::desktop_dir()`).
* Comprehensive test coverage for XDG absolute/relative path behavior and platform defaults.

### Changed

* Refactored implementation to remove duplicated logic by consolidating directory resolvers into a single `Dir`
  implementation with internal helpers.

### Deprecated

* All `*_dir()` methods are now deprecated in favor of their shorter equivalents (e.g., `Dir::desktop()`).

### Fixed

* Now correctly ignores non-absolute paths in `XDG_*` environment variables, as required by the XDG Base Directory
  Specification.

## 0.0.1 - 2025-08-08

* Initial release

[Keep a Changelog]: https://keepachangelog.com/en/1.0.0/
[Break Versioning]: https://www.taoensso.com/break-versioning

<!-- versions -->

[0.5.2]: https://github.com/aaronmallen/dir_spec/compare/0.5.1...0.5.2
[0.5.1]: https://github.com/aaronmallen/dir_spec/compare/0.5.0...0.5.1
[0.5.0]: https://github.com/aaronmallen/dir_spec/compare/0.4.0...0.5.0
[0.4.0]: https://github.com/aaronmallen/dir_spec/compare/0.3.0...0.4.0
[0.3.0]: https://github.com/aaronmallen/dir_spec/compare/0.2.0...0.3.0
[0.2.0]: https://github.com/aaronmallen/dir_spec/compare/0.1.0...0.2.0
[0.1.0]: https://github.com/aaronmallen/dir_spec/compare/0.0.1...0.1.0
