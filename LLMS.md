# LLMs

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`dir_spec` is a cross-platform Rust library for resolving XDG and platform-specific directories. It prioritizes XDG
compliance across all platforms while providing sensible platform-specific fallbacks.

## Development Commands

All development tasks are managed through `mise` (formerly rtx):

```bash
# Build the project
mise run build  # or: mise run b

# Run linting and formatting checks
mise run lint   # or: mise run l

# Run tests
mise run test:unit      # Unit tests with cargo nextest
mise run test:coverage  # Tests with code coverage

# Security audit
mise run audit

# Clean build artifacts
mise run clean

# Project setup
mise run setup
```

## Architecture

The library is organized with platform-specific modules:

- `src/lib.rs` - Main library entry point and public API
- `src/xdg.rs` - XDG Base Directory Specification implementation
- `src/linux.rs` - Linux-specific directory resolution
- `src/macos.rs` - macOS-specific directory resolution  
- `src/windows.rs` - Windows-specific directory resolution

Each platform module implements directory resolution logic that:

1. First checks for XDG environment variables
2. Falls back to platform-specific defaults if XDG vars aren't set

## Testing Strategy

- Tests are embedded in source files using `#[cfg(test)]` modules
- Uses `cargo nextest` for faster test execution
- Each platform module contains platform-specific tests
- Tests verify both XDG compliance and platform-specific fallbacks

## Key Development Notes

- **Zero dependencies** - Only uses Rust's standard library
- **Clippy pedantic** - Strict linting is enforced via `.clippy.toml`
- **Platform compatibility** - Currently supports Linux and macOS (Windows support temporarily removed from CI)
- **Type safety** - All directory methods return `Option<PathBuf>`
