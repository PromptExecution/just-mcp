# Contributing to just-mcp

Thank you for your interest in contributing to just-mcp! This document provides guidelines for contributing to the project, including how to maintain the npm and pip packages.

## Development Setup

### Prerequisites
- Rust 1.82+ with cargo
- Node.js 14+ (for npm package development)
- Python 3.8+ (for pip package development)
- `just` command runner

### Building from Source
```bash
git clone https://github.com/promptexecution/just-mcp
cd just-mcp
cargo build --release
```

### Running Tests
```bash
just test
# or
cargo test
```

## Package Maintenance

### NPM Package

The npm package is located in the `npm/` directory and provides a wrapper around the pre-built just-mcp binary.

#### Publishing to npm

1. Ensure binaries are built and uploaded to GitHub Releases for the target version
2. Update version in `npm/package.json` to match the release version
3. Test the package locally:
   ```bash
   cd npm
   npm pack
   npm install -g just-mcp-0.1.0.tgz
   just-mcp --stdio
   ```
4. Publish to npm:
   ```bash
   cd npm
   npm publish
   ```

#### How the npm Package Works

The npm package:
1. Downloads the appropriate pre-built binary during `npm install` (via `install.js`)
2. Detects the user's platform (Linux, macOS, Windows) and architecture (x64, arm64)
3. Downloads the binary from GitHub Releases
4. Falls back to building from source with cargo if binary download fails
5. Provides a `just-mcp` command that wraps the binary

### Python/pip Package

The Python package is located in the `python/` directory and provides a similar wrapper.

#### Publishing to PyPI

1. Ensure binaries are built and uploaded to GitHub Releases for the target version
2. Update version in `python/pyproject.toml` to match the release version
3. Build and test the package locally:
   ```bash
   cd python
   pip install build
   python -m build
   pip install dist/just_mcp-0.1.0-py3-none-any.whl
   just-mcp --stdio
   ```
4. Publish to PyPI:
   ```bash
   pip install twine
   python -m twine upload dist/*
   ```

#### How the Python Package Works

The Python package:
1. Downloads the appropriate pre-built binary on first run (via `__init__.py`)
2. Detects the user's platform and architecture
3. Downloads the binary from GitHub Releases
4. Caches the binary for subsequent runs
5. Provides a `just-mcp` command-line entry point

## Binary Distribution

Pre-built binaries are automatically created by the `build-binaries.yml` GitHub Actions workflow when a new release is created.

### Supported Platforms
- Linux x86_64 (`x86_64-unknown-linux-gnu`)
- Linux aarch64 (`aarch64-unknown-linux-gnu`)
- macOS x86_64 (`x86_64-apple-darwin`)
- macOS aarch64/Apple Silicon (`aarch64-apple-darwin`)
- Windows x86_64 (`x86_64-pc-windows-msvc`)

### Manual Binary Build

To manually build for a specific target:
```bash
# Install target
rustup target add x86_64-unknown-linux-gnu

# Build
cargo build --release --target x86_64-unknown-linux-gnu

# Create archive
cd target/x86_64-unknown-linux-gnu/release
tar czf just-mcp-x86_64-unknown-linux-gnu.tar.gz just-mcp
```

## Release Process

1. **Make changes** following conventional commit format:
   - `feat:` for new features
   - `fix:` for bug fixes
   - `docs:` for documentation changes
   - `chore:` for maintenance tasks

2. **CI Pipeline** runs automatically on push:
   - Runs tests
   - Checks formatting with `cargo fmt`
   - Runs clippy lints
   - Validates commits

3. **Release Pipeline** (on main branch):
   - Creates GitHub release with changelog
   - Publishes to crates.io
   - Builds binaries for all platforms

4. **Package Publishing** (manual):
   - Update npm package version and publish
   - Update Python package version and publish

## Code Style

- Use `cargo fmt` for Rust code formatting
- Follow Rust naming conventions
- Add tests for new features
- Update documentation as needed

## Questions?

If you have questions about contributing or maintaining packages, please:
- Open an issue on GitHub
- Join discussions in existing issues
- Contact the maintainers

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
