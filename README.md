# dir_spec

A cross-platform Rust library for resolving XDG and platform-specific directories with proper fallbacks.

## Why Another Directory Library?

Most existing directory libraries (like `dirs`) ignore XDG environment variables on macOS and Windows, defaulting to
platform-specific locations even when users have explicitly set XDG variables. This crate prioritizes XDG compliance
across all platforms while providing sensible platform-specific fallbacks.

## Features

- **XDG-first approach**: Respects XDG environment variables on all platforms
- **Platform-aware fallbacks**: Uses native conventions when XDG variables aren't set
- **Cross-platform**: Works on Linux, macOS, and Windows
- **Zero dependencies**: Only uses `std` library
- **Type-safe**: Returns `Option<PathBuf>` for simple error handling

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
dir_spec = "0.2.0"
```

Basic usage:

```rust
use dir_spec::Dir;

fn main() {
  // Get config directory (respects XDG_CONFIG_HOME if set)
  if let Some(config_dir) = Dir::config_home() {
    println!("Config: {}", config_dir.display());
  }

  // Get cache directory (respects XDG_CACHE_HOME if set)
  if let Some(cache_dir) = Dir::cache_home() {
    println!("Cache: {}", cache_dir.display());
  }

  // Get user's home directory
  if let Some(home_dir) = Dir::home() {
    println!("Home: {}", home_dir.display());
  }
}
```

## Supported Directories

| Method            | XDG Variable           | Linux Default        | macOS Default                    | Windows Default             |
|-------------------|------------------------|----------------------|----------------------------------|-----------------------------|
| `bin_home()`      | `XDG_BIN_HOME`         | `~/.local/bin`       | `~/.local/bin`                   | `%LOCALAPPDATA%\Programs`   |
| `cache_home()`    | `XDG_CACHE_HOME`       | `~/.cache`           | `~/Library/Caches`               | `%LOCALAPPDATA%`            |
| `config_home()`   | `XDG_CONFIG_HOME`      | `~/.config`          | `~/Library/Application Support`  | `%APPDATA%`                 |
| `config_local()`  | —                      | `~/.config`¹         | `~/Library/Application Support`¹ | `%LOCALAPPDATA%`            |
| `data_home()`     | `XDG_DATA_HOME`        | `~/.local/share`     | `~/Library/Application Support`  | `%APPDATA%`                 |
| `data_local()`    | —                      | `~/.local/share`¹    | `~/Library/Application Support`¹ | `%LOCALAPPDATA%`            |
| `desktop()`       | `XDG_DESKTOP_DIR`      | `~/Desktop`          | `~/Desktop`                      | `%USERPROFILE%\Desktop`     |
| `documents()`     | `XDG_DOCUMENTS_DIR`    | `~/Documents`        | `~/Documents`                    | `%USERPROFILE%\Documents`   |
| `downloads()`     | `XDG_DOWNLOAD_DIR`     | `~/Downloads`        | `~/Downloads`                    | `%USERPROFILE%\Downloads`   |
| `fonts()`         | —                      | `~/.local/share/fonts` | `~/Library/Fonts`              | `None`²                     |
| `home()`          | `HOME` / `USERPROFILE` | `$HOME`              | `$HOME`                          | `%USERPROFILE%`             |
| `music()`         | `XDG_MUSIC_DIR`        | `~/Music`            | `~/Music`                        | `%USERPROFILE%\Music`       |
| `pictures()`      | `XDG_PICTURES_DIR`     | `~/Pictures`         | `~/Pictures`                     | `%USERPROFILE%\Pictures`    |
| `preferences()`   | —                      | `~/.config`¹         | `~/Library/Preferences`          | `%APPDATA%`¹                |
| `publicshare()`   | `XDG_PUBLICSHARE_DIR`  | `~/Public`           | `~/Public`                       | `C:\Users\Public`           |
| `runtime()`       | `XDG_RUNTIME_DIR`      | `$TMPDIR` or `/tmp`  | `$TMPDIR` or `/tmp`              | `%TEMP%`                    |
| `state_home()`    | `XDG_STATE_HOME`       | `~/.local/state`     | `~/Library/Application Support`  | `%LOCALAPPDATA%`            |
| `templates()`     | `XDG_TEMPLATES_DIR`    | `~/Templates`        | `~/Templates`                    | `%USERPROFILE%\Templates`   |
| `videos()`        | `XDG_VIDEOS_DIR`       | `~/Videos`           | `~/Movies`                       | `%USERPROFILE%\Videos`      |

**Notes:**

1. Same as the corresponding `*_home()` function on non-Windows platforms
2. Returns `None` on Windows as there is no standard user fonts directory

## Directory Types Explained

### Config vs. Config Local

- **`config_home()`**: Roaming config directory (synced across machines on Windows)
- **`config_local()`**: Local config directory (machine-specific, not synced)

### Data vs. Data Local

- **`data_home()`**: Roaming data directory (synced across machines on Windows)
- **`data_local()`**: Local data directory (machine-specific, not synced)

### Config vs. Preferences

- **`config_home()`**: General application configuration
- **`preferences()`**: Platform-specific preferences (macOS: `.plist` files via Apple APIs)

### Fonts

- **`fonts()`**: User-installed fonts directory
- Returns `None` on Windows as there's no standard user fonts directory

## XDG Environment Variable Priority

This crate always checks XDG environment variables first, regardless of platform:

```rust
// This will use XDG_CONFIG_HOME if set, even on macOS/Windows
export XDG_CONFIG_HOME="/custom/config/path"

let config = Dir::config_home(); // Returns Some("/custom/config/path")
```

If XDG variables aren't set, the crate falls back to platform-appropriate defaults.

## Cross-Platform Behavior

### Linux

Follows XDG Base Directory Specification defaults when XDG variables aren't set.

### macOS

- Respects XDG variables if set (common among CLI tool users)
- Falls back to native macOS locations (`~/Library/Application Support`, etc.)
- Uses `~/Movies` for videos (not `~/Videos`)
- `preferences()` points to `~/Library/Preferences` for `.plist` files

### Windows

- Respects XDG variables if set
- Falls back to Windows conventions (`%APPDATA%`, `%LOCALAPPDATA%`, etc.)
- Public directory points to system-wide `C:\Users\Public`
- `config_local()` and `data_local()` use `%LOCALAPPDATA%` for non-roaming storage
- `fonts()` returns `None` (no standard user fonts directory)

## Error Handling

All methods return `Option<PathBuf>`. Methods return `None` when:

- Home directory cannot be determined
- Required environment variables are missing (Windows-specific cases)
- Platform-specific directory resolution fails
- Directory doesn't exist on the platform (e.g., `fonts()` on Windows)

```rust
match Dir::config_home() {
    Some(path) => println!("Config dir: {}", path.display()),
    None => eprintln!("Failed to get config dir"),
}

// Or using if-let
if let Some(config_path) = Dir::config_home() {
    println!("Config dir: {}", config_path.display());
}

// For fallback handling
let config_dir = Dir::config_home().unwrap_or_else(|| {
    // Fallback to current directory or panic, depending on your needs
    std::env::current_dir().expect("Cannot determine current directory")
});
```

## Dependencies

None! This crate only uses Rust's standard library.

## License

Licensed under the [MIT LICENSE](./LICENSE)

## Contributing

Contributions are welcome! Please ensure:

1. All platforms are tested
2. XDG compliance is maintained
3. Platform-specific fallbacks follow native conventions
4. New methods include appropriate documentation

## References

- [XDG Base Directory Specification][xdg-spec]
- [Apple File System Programming Guide][apple-guide]
- [Windows Known Folder IDs][windows-folders]

[xdg-spec]: https://specifications.freedesktop.org/basedir-spec/latest/
[apple-guide]:
https://developer.apple.com/library/archive/documentation/FileManagement/Conceptual/FileSystemProgrammingGuide/
[windows-folders]: https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid
