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
- **Zero dependencies**: Only uses `std` and `libc` for Unix systems
- **Type-safe**: Returns `Result<PathBuf>` for proper error handling

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
dir_spec = "0.1.0"
```

Basic usage:

```rust
use dir_spec::Dir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get config directory (respects XDG_CONFIG_HOME if set)
    let config_dir = Dir::config_home()?;
    println!("Config: {}", config_dir.display());
    
    // Get cache directory (respects XDG_CACHE_HOME if set)
    let cache_dir = Dir::cache_home()?;
    println!("Cache: {}", cache_dir.display());
    
    // Get user's home directory
    let home_dir = Dir::home()?;
    println!("Home: {}", home_dir.display());
    
    Ok(())
}
```

## Supported Directories

| Method              | XDG Variable           | Linux Default    | macOS Default                   | Windows Default            |
|---------------------|------------------------|------------------|---------------------------------|----------------------------|
| `bin_home()`        | `XDG_BIN_HOME`         | `~/.local/bin`   | `~/.local/bin`                  | `%LOCALAPPDATA%\Programs`  |
| `cache_home()`      | `XDG_CACHE_HOME`       | `~/.cache`       | `~/Library/Caches`              | `%LOCALAPPDATA%`           |
| `config_home()`     | `XDG_CONFIG_HOME`      | `~/.config`      | `~/Library/Application Support` | `%APPDATA%`                |
| `data_home()`       | `XDG_DATA_HOME`        | `~/.local/share` | `~/Library/Application Support` | `%APPDATA%`                |
| `desktop_dir()`     | `XDG_DESKTOP_DIR`      | `~/Desktop`      | `~/Desktop`                     | `%USERPROFILE%\Desktop`    |
| `documents_dir()`   | `XDG_DOCUMENTS_DIR`    | `~/Documents`    | `~/Documents`                   | `%USERPROFILE%\Documents`  |
| `download_dir()`    | `XDG_DOWNLOAD_DIR`     | `~/Downloads`    | `~/Downloads`                   | `%USERPROFILE%\Downloads`  |
| `music_dir()`       | `XDG_MUSIC_DIR`        | `~/Music`        | `~/Music`                       | `%USERPROFILE%\Music`      |
| `pictures_dir()`    | `XDG_PICTURES_DIR`     | `~/Pictures`     | `~/Pictures`                    | `%USERPROFILE%\Pictures`   |
| `publicshare_dir()` | `XDG_PUBLICSHARE_DIR`  | `~/Public`       | `~/Public`                      | `C:\Users\Public`          |
| `runtime_dir()`     | `XDG_RUNTIME_DIR`      | `/run/user/$UID` | `$TMPDIR` or `/tmp`             | `%TEMP%`                   |
| `state_home()`      | `XDG_STATE_HOME`       | `~/.local/state` | `~/Library/Application Support` | `%LOCALAPPDATA%`           |
| `templates_dir()`   | `XDG_TEMPLATES_DIR`    | `~/Templates`    | `~/Templates`                   | `%USERPROFILE%\Templates`  |
| `videos_dir()`      | `XDG_VIDEOS_DIR`       | `~/Videos`       | `~/Movies`                      | `%USERPROFILE%\Videos`     |
| `home()`            | `HOME` / `USERPROFILE` | `$HOME`          | `$HOME`                         | `%USERPROFILE%`            |

## XDG Environment Variable Priority

This crate always checks XDG environment variables first, regardless of platform:

```rust
// This will use XDG_CONFIG_HOME if set, even on macOS/Windows
export XDG_CONFIG_HOME="/custom/config/path"

let config = Dir::config_home()?; // Returns "/custom/config/path"
```

If XDG variables aren't set, the crate falls back to platform-appropriate defaults.

## Cross-Platform Behavior

### Linux

Follows XDG Base Directory Specification defaults when XDG variables aren't set.

### macOS

- Respects XDG variables if set (common among CLI tool users)
- Falls back to native macOS locations (`~/Library/Application Support`, etc.)
- Uses `~/Movies` for videos (not `~/Videos`)

### Windows

- Respects XDG variables if set
- Falls back to Windows conventions (`%APPDATA%`, `%LOCALAPPDATA%`, etc.)
- Public directory points to system-wide `C:\Users\Public`

## Error Handling

All methods return `Result<PathBuf, eyre::Error>`. Common error cases:

- Home directory cannot be determined
- Required environment variables are missing (Windows-specific cases)
- Path contains invalid UTF-8 (Unix systems only)

```rust
match Dir::config_home() {
    Ok(path) => println!("Config dir: {}", path.display()),
    Err(e) => eprintln!("Failed to get config dir: {}", e),
}
```

## Dependencies

- `eyre`: For error handling
- `libc`: For Unix systems (accessing passwd database when `$HOME` isn't set)

## License

Licensed under the [MIT LICENSE](./LICENSE)

at your option.

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
