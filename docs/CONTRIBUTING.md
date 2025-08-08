# Contributing to dir_spec

Thank you for your interest in contributing to dir_spec! This guide will help you get set up and understand our
development workflow.

## Quick Start

### Prerequisites

- **Rust**: Install via [rustup](https://rustup.rs/)
- **Git**: For version control

### Development Setup (Recommended: mise)

We **strongly recommend** using [mise](https://mise.jdx.dev/) for tool management as it automatically handles all
dependencies and toolchain setup:

1. **Clone the repository**

   ```bash
   git clone https://github.com/aaronmallen/dir_spec.git
   cd dir_spec
   ```

2. **Install mise**

   ```bash
   curl https://mise.jdx.dev/install.sh | sh
   ```

3. **Install tools and dependencies**

   ```bash
   mise install
   mise run setup
   ```

4. **Verify setup**

   ```bash
   mise run test
   mise run lint:check:rust
   ```

### Alternative: Manual Setup

If you prefer not to use mise, you can use our setup script:

1. **Install Rust toolchain**

   ```bash
   # The project will automatically use the toolchain specified in rust-toolchain.toml
   rustup component add clippy rustfmt
   ```

2. **Install development tools**

   ```bash
   ./bin/setup
   ```

3. **Verify setup**

   ```bash
   ./bin/test
   ./bin/lint/check/rust
   ```

## Development Workflow

### Running Tests

```bash
# Recommended: Use mise
mise run test

# Without mise: Use bin scripts
./bin/test
```

### Code Formatting and Linting

We use several tools to maintain code quality:

```bash
# Recommended: Use mise
mise run lint:check:rust  # Check formatting, imports, and clippy
mise run lint:fix:rust    # Auto-fix formatting and imports
mise run lint:check:md    # Check markdown formatting
mise run lint:fix:md      # Auto-fix markdown formatting

# Without mise: Use bin scripts
./bin/lint/check/rust     # Check formatting, imports, and clippy
./bin/lint/fix/rust       # Auto-fix formatting and imports
./bin/lint/check/markdown # Check markdown formatting
./bin/lint/fix/markdown   # Auto-fix markdown formatting

# Run all checks at once
./bin/lint/_default       # Check everything
./bin/lint/fix/_default   # Fix everything
```

### Running All CI Checks

To run the same checks that CI runs:

```bash
# Recommended: Use mise
mise run ci

# Without mise: Use bin scripts
./bin/ci
```

### Other Useful Commands

```bash
# Build the project
./bin/build

# Check for dependency updates and security issues
./bin/audit

# Clean build artifacts
./bin/clean
```

**Our linting includes:**

- **cargo-sort**: Ensures `Cargo.toml` dependencies are sorted
- **rustfmt**: Code formatting (configured in `rustfmt.toml`)
- **clippy**: Lint checks for common mistakes and improvements
- **markdownlint**: Markdown file linting

### Code Style

- **Formatting**: We use `rustfmt` with custom settings (see `rustfmt.toml`)
- **Imports**: Sorted and grouped by `StdExternalCrate`
- **Line length**: 120 characters max
- **Indentation**: 2 spaces (no tabs)

### Making Changes

#### Create a feature branch

   ```bash
   git checkout -b feature/your-feature-name
   ```

#### *Make your changes

- Add tests for new functionality
- Update documentation as needed
- Follow existing code patterns

#### Test your changes

   ```bash
   # Recommended: Use mise
   mise run test
   mise run lint:check:rust
   
   # Without mise: Use bin scripts
   ./bin/test
   ./bin/lint/check/rust
   ```

#### Commit your changes

   ```bash
   git add .
   git commit -m "feat: add support for XYZ"
   ```

#### Push and create a PR

   ```bash
   git push origin feature/your-feature-name
   ```

## Project Structure

```text
dir_spec/
├── src/
│   ├── lib.rs          # Public API exports
│   └── dir.rs          # Core directory resolution logic
├── bin/
│   ├── audit           # Dependency security and update checks
│   ├── build           # Build the project
│   ├── ci              # Run all CI checks
│   ├── clean           # Clean build artifacts
│   ├── lint/
│   │   ├── _default    # Run all lint checks
│   │   ├── check/
│   │   │   ├── markdown # Check markdown formatting
│   │   │   └── rust    # Check rust formatting/linting
│   │   └── fix/
│   │       ├── _default # Auto-fix all issues
│   │       ├── markdown # Auto-fix markdown
│   │       └── rust    # Auto-fix rust formatting
│   ├── setup           # Install development dependencies
│   └── test            # Run tests
├── rust-toolchain.toml # Rust version specification
├── rustfmt.toml        # Code formatting rules
└── Cargo.toml          # Project configuration
```

## Testing Guidelines

### Platform Coverage

This crate supports Linux, macOS, and Windows. When adding new features:

- **Test on your platform** during development
- **Consider all platforms** in your implementation
- **Use `#[cfg(target_os = "...")]`** for platform-specific code
- **CI will test** on all platforms automatically

### Test Categories

- **Unit tests**: Test individual functions and edge cases
- **Integration tests**: Test the full API surface
- **Platform-specific tests**: Verify platform conventions are followed

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_home_xdg_override() {
        // Test XDG environment variable override
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn test_config_home_linux_default() {
        // Test Linux-specific defaults
    }
}
```

## XDG Compliance

This crate prioritizes XDG compliance across all platforms:

### Key Principles

1. **XDG variables take precedence** on all platforms
2. **Platform conventions** are used as fallbacks
3. **Consistent behavior** across operating systems
4. **Proper error handling** for missing directories

### Adding New Directories

When adding support for new directories:

1. **Research platform conventions**

- Linux: Follow XDG Base Directory Specification
- macOS: Use native `~/Library/*` locations
- Windows: Use appropriate `%APPDATA%` or `%LOCALAPPDATA%` paths

1. **Implement XDG-first logic**

   ```rust
   pub fn new_directory() -> Result<PathBuf> {
       if let Ok(xdg_var) = env::var("XDG_NEW_HOME") {
           return Ok(PathBuf::from(xdg_var));
       }
       
       // Platform-specific fallbacks
       #[cfg(target_os = "linux")]
       { /* Linux default */ }
       
       #[cfg(target_os = "macos")]
       { /* macOS default */ }
       
       #[cfg(target_os = "windows")]
       { /* Windows default */ }
   }
   ```

2. **Update documentation**

- Add to the table in README.md
- Include examples in doc comments

## Pull Request Guidelines

### Before Submitting

- [ ] Code passes all tests (`mise run test` or `./bin/test`)
- [ ] Code passes all lints (`mise run lint:check:rust` or `./bin/lint/check/rust`)
- [ ] Documentation is updated if needed
- [ ] Platform-specific behavior is documented
- [ ] XDG compliance is maintained

### PR Description

Please include:

- **What**: Brief description of changes
- **Why**: Motivation for the change
- **Platform impact**: Any platform-specific considerations
- **Testing**: How you tested the changes

### Review Process

1. **Automated checks** must pass (CI)
2. **Code review** by maintainers
3. **Testing** on multiple platforms (via CI)
4. **Merge** once approved

## Getting Help

- **Issues**: Open an issue for bugs or feature requests
- **Discussions**: Use GitHub Discussions for questions
- **Documentation**: Check the README.md and code comments

## References

Understanding these specifications will help with contributions:

- [XDG Base Directory Specification][xdg-spec]
- [Apple File System Programming Guide][apple-guide]
- [Windows Known Folder IDs][windows-folders]

[xdg-spec]: https://specifications.freedesktop.org/basedir-spec/latest/
[apple-guide]:
  https://developer.apple.com/library/archive/documentation/FileManagement/Conceptual/FileSystemProgrammingGuide/
[windows-folders]: https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid

## License

By contributing to dir_spec, you agree that your contributions will be licensed under the MIT License.
