# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to [Break Versioning].

## [UNRELEASED]

### Changed

* **BREAKING**: All methods now return `Option<PathBuf>` instead of `Result<PathBuf>`
* **BREAKING**: `home()` method now uses `std::env::home_dir()` directly (was previously deprecated, now undeprecated)
* Simplified error handling by removing `eyre` dependency from public API
* Updated documentation and examples to use Option pattern matching

### Removed

* **BREAKING**: Removed `eyre` dependency from public API
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

[Unreleased]: https://github.com/aaronmallen/dir_spec/compare/0.1.0...HEAD
[0.1.0]: https://github.com/aaronmallen/dir_spec/compare/0.0.1...0.1.0
