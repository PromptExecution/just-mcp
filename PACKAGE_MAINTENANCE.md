# Package Maintenance Quick Reference

Quick reference for maintaining npm and pip packages for just-mcp.

## Release Checklist

### 1. Create GitHub Release (Automated via CI/CD)

When changes are pushed to `main` branch with conventional commits:
- CI runs tests
- Release workflow creates GitHub release with version bump
- Binary build workflow creates multi-platform binaries
- Binaries are uploaded to GitHub release

### 2. Update Package Versions

```bash
# Use the helper script
./scripts/publish-packages.sh 0.1.0

# Or manually:
# npm
cd npm && npm version 0.1.0 --no-git-tag-version

# Python
# Edit python/pyproject.toml and python/just_mcp/__init__.py
```

### 3. Test Packages Locally

See [TESTING.md](TESTING.md) for comprehensive testing instructions.

Quick tests:
```bash
# npm
cd npm && npm pack && npm install -g just-mcp-0.1.0.tgz

# Python
cd python && python -m build && pip install dist/just_mcp-0.1.0-py3-none-any.whl
```

### 4. Publish to npm

```bash
cd npm

# Login to npm (first time only)
npm login

# Publish
npm publish

# Verify
npm info just-mcp
```

### 5. Publish to PyPI

```bash
cd python

# Install twine (first time only)
pip install twine

# Build if not already built
python -m build

# Upload to PyPI
python -m twine upload dist/*

# Verify
pip index versions just-mcp
```

### 6. Verify Installation

```bash
# npm
npx just-mcp@latest --version

# Python
uvx just-mcp@latest --version
```

### 7. Update Documentation

- Update main README.md with new version numbers if needed
- Add to CHANGELOG.md if not auto-generated
- Update any version-specific documentation

## Common Tasks

### Update Binary Download URL

If GitHub release structure changes, update:
- `npm/install.js` - `downloadUrl` construction
- `python/just_mcp/__init__.py` - `download_binary()` function

### Add New Platform Support

1. Add to `PLATFORM_MAP` in:
   - `npm/install.js`
   - `python/just_mcp/__init__.py`

2. Add to `.github/workflows/build-binaries.yml` matrix

3. Test on new platform

### Debug Installation Issues

Enable verbose logging:

```bash
# npm
npm install --loglevel verbose just-mcp

# Python  
pip install --verbose just-mcp
```

Check package contents:
```bash
# npm
npm pack just-mcp
tar -tzf just-mcp-0.1.0.tgz

# Python
unzip -l dist/just_mcp-0.1.0-py3-none-any.whl
```

## Version Synchronization

Keep versions synchronized across:
- `Cargo.toml` (main crate)
- `just-mcp-lib/Cargo.toml` (library crate)
- `npm/package.json`
- `python/pyproject.toml`
- `python/just_mcp/__init__.py`

Use the `publish-packages.sh` script to update npm and Python versions automatically.

## Publishing Permissions

### npm
- Requires npm account with publish permissions for `just-mcp` package
- First publish: Package name must be available
- Scope packages: Can use `@promptexecution/just-mcp` if needed

### PyPI
- Requires PyPI account with publish permissions for `just-mcp` package
- First publish: Package name must be available
- Can use TestPyPI for testing: `python -m twine upload --repository testpypi dist/*`

## Rollback Procedure

### npm
```bash
# Deprecate specific version
npm deprecate just-mcp@0.1.0 "Bug in binary download"

# Publish fix as patch version
npm version patch
npm publish
```

### PyPI
PyPI does not allow deleting published versions. Options:
1. Publish a new fixed version (recommended)
2. Yank the release (makes it unavailable for new installs but keeps it for existing)

## Support Channels

For issues with package distribution:
- GitHub Issues: https://github.com/promptexecution/just-mcp/issues
- Check npm package page: https://www.npmjs.com/package/just-mcp
- Check PyPI package page: https://pypi.org/project/just-mcp/

## Automation Opportunities

Consider automating in the future:
- Auto-publish to npm after GitHub release
- Auto-publish to PyPI after GitHub release
- Auto-sync version numbers across all package files
- Automated integration testing in CI/CD
