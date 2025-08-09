# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to [Break Versioning].

## [Unreleased]

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
