# pkgx Package Setup Guide

A comprehensive checklist for adding pkgx support to your project.

## Prerequisites

- [ ] Read the pkgx pantry wiki: https://github.com/pkgxdev/pantry/wiki
- [ ] Review the pantry repository: https://github.com/pkgxdev/pantry
- [ ] Browse existing packages for examples: https://github.com/pkgxdev/pantry/tree/main/projects

## Phase 1: Understand Requirements

### 1.1 Core Concepts
- [ ] Understand that packages must be **relocatable** (movable between directories)
- [ ] Know that the build script MUST install to `{{prefix}}` or it will fail
- [ ] Understand the three required sections: `distributable`, `versions`, `build`
- [ ] Know that a comprehensive `test` section is required

### 1.2 Package Naming
- [ ] Determine your package name (use homepage domain or GitHub path)
  - Example: `github.com/user/repo` if no homepage exists
  - Example: `gnu.org/wget` for projects with homepages
- [ ] Check if name already exists: https://github.com/pkgxdev/pantry/tree/main/projects

## Phase 2: Prepare Your Project

### 2.1 Release Artifacts
- [ ] Ensure your project has a release workflow that creates GitHub releases
- [ ] Verify releases include binary artifacts (`.tar.gz` or similar)
- [ ] Check artifact naming is consistent and includes platform/architecture
  - Example: `myproject-x86_64-unknown-linux-gnu.tar.gz`
  - Example: `myproject-aarch64-apple-darwin.tar.gz`

### 2.2 CI/CD Integration
- [ ] If using GitHub Actions, ensure binary build workflow triggers on `release: types: [created]`
- [ ] **IMPORTANT**: Add explicit workflow trigger if using default `GITHUB_TOKEN`
  ```yaml
  - name: Trigger binary builds
    run: |
      gh workflow run build-binaries.yml -f tag="${NEW_TAG}"
  ```
- [ ] Verify binaries are uploaded to the release (not just artifacts)

### 2.3 Test a Release
- [ ] Create a test release manually
- [ ] Verify all platform binaries are built and attached
- [ ] Download and test each binary manually
- [ ] Confirm binary works with `--version` or equivalent

## Phase 3: Create package.yml

### 3.1 Initialize Structure
- [ ] Fork the pantry repo: https://github.com/pkgxdev/pantry/fork
- [ ] Clone your fork locally
- [ ] Use `bk init your-domain.com/package` to create template (if brewkit available)
- [ ] OR manually create `projects/your-domain.com/package-name/package.yml`

### 3.2 Write Distributable Section
```yaml
distributable:
  url: https://github.com/user/repo/archive/refs/tags/v{{version}}.tar.gz
  strip-components: 1  # Strips top-level directory from tarball
```

**Checklist:**
- [ ] URL uses `{{version}}` template variable
- [ ] URL points to source tarball (for source builds) OR release binaries (for binary packages)
- [ ] Test URL manually by replacing `{{version}}` with actual version

### 3.3 Write Versions Section
```yaml
versions:
  github: user/repo/tags  # Auto-discover from GitHub tags
  strip: /^v/             # Remove 'v' prefix from version tags
```

**Checklist:**
- [ ] If using GitHub tags, verify format matches your tags
- [ ] If manually listing versions, include at least one stable release
- [ ] Test that version discovery works

### 3.4 Write Build Section

**For binary distributions (recommended if you ship pre-built binaries):**
```yaml
build:
  dependencies:
    curl.se: '*'
  working-directory: ${{prefix}}
  script: |
    case "{{hw.platform}}+{{hw.arch}}" in
      linux+x86-64)
        target="x86_64-unknown-linux-gnu"
        ;;
      linux+aarch64)
        target="aarch64-unknown-linux-gnu"
        ;;
      darwin+x86-64)
        target="x86_64-apple-darwin"
        ;;
      darwin+aarch64)
        target="aarch64-apple-darwin"
        ;;
      *)
        echo "unsupported platform: {{hw.platform}}+{{hw.arch}}" >&2
        exit 1
        ;;
    esac

    archive="myproject-${target}.tar.gz"
    curl -sSfL -o "$archive" "https://github.com/user/repo/releases/download/v{{version}}/${archive}"
    tar -xzf "$archive"
    mkdir -p {{ prefix }}/bin
    mv myproject {{ prefix }}/bin/myproject
```

**Checklist:**
- [ ] All target platforms are covered
- [ ] URLs match your actual release artifact names
- [ ] Script extracts binary to `{{ prefix }}/bin/`
- [ ] Script handles both Unix and Windows naming (`.exe` suffix)

**For source builds:**
```yaml
build:
  dependencies:
    rust-lang.org: '*'  # or whatever your build tools need
  script: |
    cargo build --release
    mkdir -p {{ prefix }}/bin
    mv target/release/myproject {{ prefix }}/bin/
```

### 3.5 Add Provides Section
```yaml
provides:
  - bin/myproject
```

**Checklist:**
- [ ] Lists all executables installed by the package
- [ ] Paths are relative to `{{ prefix }}`

### 3.6 Write Test Section
```yaml
test: |
  myproject --version
  myproject --help
```

**Checklist:**
- [ ] Test actually validates the binary works
- [ ] Test uses commands that will succeed reliably
- [ ] Consider testing core functionality, not just `--version`

### 3.7 Add Metadata (Optional but Recommended)
```yaml
description: Brief description of your project
homepage: https://github.com/user/repo
license: MIT
```

## Phase 4: Local Testing

### 4.1 Install brewkit
- [ ] Install brewkit: https://github.com/pkgxdev/brewkit
  ```bash
  pkgx install brewkit
  ```

### 4.2 Test Build Locally
- [ ] Navigate to pantry clone directory
- [ ] Set pantry path: `export PKGX_PANTRY_PATH=$(pwd)`
- [ ] Initialize package: `bk init your-domain.com/package`
- [ ] Build package: `bk build your-domain.com/package`
- [ ] Check that build succeeds and installs to `./builds/`

### 4.3 Test Installation
- [ ] Test with pkgx: `pkgx your-domain.com/package --version`
- [ ] Verify correct version is detected
- [ ] Verify binary executes correctly
- [ ] Test on multiple platforms if possible (Docker, CI, etc.)

### 4.4 Test Validation
- [ ] Run test script: `bk test your-domain.com/package`
- [ ] Ensure all tests pass
- [ ] Add more comprehensive tests if needed

## Phase 5: Submit to Pantry

### 5.1 Prepare Submission
- [ ] Review the contribution guide: https://github.com/pkgxdev/pantry/blob/main/CONTRIBUTING.md
- [ ] Ensure your fork is up to date with upstream
- [ ] Create a feature branch: `git checkout -b add-your-package`
- [ ] Commit your `package.yml`
  ```bash
  git add projects/your-domain.com/package-name/package.yml
  git commit -m "add: your-domain.com/package-name"
  ```

### 5.2 Create Pull Request
- [ ] Push to your fork: `git push origin add-your-package`
- [ ] Open PR at: https://github.com/pkgxdev/pantry/pulls
- [ ] Fill out PR template completely
- [ ] Reference any related issues or discussions
- [ ] Wait for CI checks to pass
- [ ] Address any reviewer feedback

### 5.3 PR Checklist (for reviewers)
- [ ] Package name follows conventions (domain-based)
- [ ] All required sections present (`distributable`, `versions`, `build`, `test`)
- [ ] Build script installs to `{{ prefix }}`
- [ ] Test script thoroughly validates functionality
- [ ] Package is relocatable (no hardcoded paths)
- [ ] Metadata is accurate (description, license, homepage)

## Phase 6: Post-Merge Verification

### 6.1 Wait for Pantry Update
- [ ] PR is merged to main
- [ ] Wait for pantry CDN to sync (usually minutes)

### 6.2 Test Installation
- [ ] Clear local pkgx cache: `rm -rf ~/.pkgx`
- [ ] Test fresh install: `pkgx your-package-name --version`
- [ ] Verify correct version is installed
- [ ] Test on clean system (Docker, fresh VM, etc.)

### 6.3 Test Auto-Updates
- [ ] Create a new release of your project
- [ ] Wait for pantry to detect new version (automated)
- [ ] Verify `pkgx your-package-name` uses new version
- [ ] If using GitHub tags with `versions: github:`, updates are automatic

## Phase 7: Maintenance

### 7.1 Monitor Issues
- [ ] Watch pantry repo for issues mentioning your package
- [ ] Subscribe to notifications for your package directory

### 7.2 Update Package
- [ ] If package.yml needs changes, fork and submit new PR
- [ ] Test changes locally before submitting
- [ ] Document breaking changes in PR description

### 7.3 Ensure Release Automation
- [ ] Verify each new release includes binary artifacts
- [ ] Monitor release workflow for failures
- [ ] Keep binary naming conventions consistent

## Common Pitfalls

### Binary Distribution Issues
- ❌ **Binaries not uploaded to release**
  - GitHub Actions workflows using `GITHUB_TOKEN` don't trigger other workflows
  - Add explicit `gh workflow run` trigger in release workflow

- ❌ **Inconsistent artifact naming**
  - Binary names must match pattern in `package.yml`
  - Use consistent naming: `project-${target}.tar.gz`

- ❌ **Wrong working directory**
  - Build script must use `working-directory: ${{prefix}}`
  - Or manually `cd {{ prefix }}` in script

### Build Script Issues
- ❌ **Not installing to `{{ prefix }}`**
  - Build WILL FAIL if nothing is installed to `{{ prefix }}`
  - Always `mkdir -p {{ prefix }}/bin` and move binaries there

- ❌ **Hardcoded paths**
  - Package must be relocatable
  - Never use absolute paths like `/usr/local`

### Testing Issues
- ❌ **Weak test script**
  - `--version` is good but not sufficient
  - Test actual functionality
  - Ensure tests are deterministic

- ❌ **Missing platform support**
  - Cover Linux (x86_64, aarch64)
  - Cover macOS (x86_64, aarch64/M1)
  - Windows if applicable

## Resources

- **Main pantry repo**: https://github.com/pkgxdev/pantry
- **Pantry wiki**: https://github.com/pkgxdev/pantry/wiki
- **Contributing guide**: https://github.com/pkgxdev/pantry/blob/main/CONTRIBUTING.md
- **brewkit CLI**: https://github.com/pkgxdev/brewkit
- **pkgx homepage**: https://pkgx.sh
- **Package examples**: https://github.com/pkgxdev/pantry/tree/main/projects
- **Example Rust projects**:
  - https://github.com/pkgxdev/pantry/blob/main/projects/rust-lang.org/package.yml
  - Search for `cargo build` in pantry

## Quick Reference: package.yml Template

```yaml
distributable:
  url: https://github.com/user/repo/releases/download/v{{version}}/repo-src.tar.gz
  strip-components: 1

versions:
  github: user/repo/tags
  strip: /^v/

description: One-line description of your project
homepage: https://your-project.com
license: MIT

build:
  dependencies:
    curl.se: '*'
  working-directory: ${{prefix}}
  script: |
    # Download and extract binary for current platform
    case "{{hw.platform}}+{{hw.arch}}" in
      linux+x86-64) target="x86_64-unknown-linux-gnu" ;;
      linux+aarch64) target="aarch64-unknown-linux-gnu" ;;
      darwin+x86-64) target="x86_64-apple-darwin" ;;
      darwin+aarch64) target="aarch64-apple-darwin" ;;
      *) echo "unsupported: {{hw.platform}}+{{hw.arch}}" >&2; exit 1 ;;
    esac

    archive="myproject-${target}.tar.gz"
    curl -sSfL -o "$archive" \
      "https://github.com/user/repo/releases/download/v{{version}}/${archive}"
    tar -xzf "$archive"
    mkdir -p {{ prefix }}/bin
    mv myproject {{ prefix }}/bin/

provides:
  - bin/myproject

test: |
  myproject --version | grep {{version}}
  myproject --help
```

## Example: just-mcp Setup

See the complete working example in this repository:
- **package.yml**: `pkgx/projects/github.com/promptexecution/just-mcp/package.yml`
- **Build workflow**: `.github/workflows/build-binaries.yml`
- **Release workflow**: `.github/workflows/release.yml` (note the trigger fix)
- **README section**: Installation via pkgx
