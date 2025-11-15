# Fix for PR #5 - Conventional Commits Violation

## Problem Summary
PR #5 (branch: `copilot/maintain-npm-and-pip-installers`) is failing CI checks due to commit message format violations. The CI workflow runs `cog check --from-latest-tag` which validates that all commits follow the Conventional Commits format required by the project.

## Root Cause
One commit in the PR has the message "Initial plan" (commit SHA: `25cc1e3`) which doesn't follow the Conventional Commits format. The project requires commit messages to have a type prefix such as `feat:`, `fix:`, `docs:`, `chore:`, etc.

## CI Failure Details
- **Workflow**: `.github/workflows/ci.yml` - `lint-commits` job
- **Check**: `cog check --from-latest-tag`
- **Error**: Commit message "Initial plan" is not conventional
- **Required Format**: `<type>: <description>` (e.g., "chore: initial plan")

## Valid Commit Types
According to `cog.toml`, the following commit types are accepted:
- `feat:` - New features (minor version bump)
- `fix:` - Bug fixes (patch version bump)
- `docs:` - Documentation changes (patch version bump)
- `chore:` - Miscellaneous tasks (patch version bump)
- `ci:` - CI/CD changes (patch version bump)
- `style:` - Code formatting (patch version bump)
- `refactor:` - Code refactoring (patch version bump)
- `perf:` - Performance improvements (patch version bump)
- `test:` - Testing changes (patch version bump)
- `build:` - Build system changes (patch version bump)
- `revert:` - Reverts (patch version bump)
- `breaking:` - Breaking changes (major version bump)

## Solution: Fix the Commit Message

The commit message needs to be changed from "Initial plan" to follow the Conventional Commits format. The most appropriate fix would be "chore: initial plan" since it appears to be a planning/setup commit.

### Option 1: Interactive Rebase (Recommended for Maintainers)

```bash
# Checkout the PR branch
git checkout copilot/maintain-npm-and-pip-installers

# Fetch the latest tag
git fetch --tags

# Start interactive rebase from v0.1.0 tag  
GIT_EDITOR="sed -i 's/^pick 25cc1e3/reword 25cc1e3/'" git rebase -i v0.1.0

# When the editor opens for the commit message, change:
# From: "Initial plan"
# To:   "chore: initial plan"

# Force push the fixed branch (requires force-push permissions)
git push --force-with-lease origin copilot/maintain-npm-and-pip-installers
```

### Option 2: Automated Script (Fastest)

```bash
git checkout copilot/maintain-npm-and-pip-installers

# Create editor script
cat > /tmp/fix-commit.sh << 'EOF'
#!/bin/bash
if [[ "$1" == *"git-rebase-todo"* ]]; then
    sed -i 's/^pick 25cc1e3/reword 25cc1e3/' "$1"
elif [[ "$1" == *"COMMIT_EDITMSG"* ]]; then
    sed -i '1s/^Initial plan$/chore: initial plan/' "$1"
fi
EOF
chmod +x /tmp/fix-commit.sh

# Run rebase with the script
GIT_EDITOR=/tmp/fix-commit.sh git rebase -i v0.1.0

# Force push
git push --force-with-lease origin copilot/maintain-npm-and-pip-installers
```

### Option 3: Apply from Fixed Branch (If Available)

If you have access to a branch with the fixed commits, you can reset the PR branch to it:

```bash
# If the fixed commits are on another branch (e.g., local or another remote branch)
git checkout copilot/maintain-npm-and-pip-installers
git reset --hard <branch-with-fixed-commits>
git push --force-with-lease origin copilot/maintain-npm-and-pip-installers
```

## Verification

After applying the fix, verify it worked:

```bash
# Check that the commit message is now correct
git log --oneline v0.1.0..HEAD | grep "chore: initial plan"

# Run the cocogitto check (should pass now)
cog check --from-latest-tag

# Verify CI will pass
git push origin copilot/maintain-npm-and-pip-installers
# Then check GitHub Actions CI results
```

## Additional Notes

- The PR contains 22 commits total since v0.1.0
- All other commits already follow the Conventional Commits format
- The fix only requires changing one commit message
- No code changes are needed
- After fixing, the `lint-commits` job in CI should pass

## PR #5 Content Summary

The PR adds npm and pip package distribution with automated binary builds:
- npm package wrapper (`npm/` directory)
- Python/pip package wrapper (`python/` directory)
- GitHub Actions workflow for building cross-platform binaries
- Documentation (CONTRIBUTING.md, TESTING.md, PACKAGE_MAINTENANCE.md)
- Updated README with installation instructions

This is valuable functionality that allows easier installation of just-mcp without requiring Rust toolchain.

## References

- PR #5: https://github.com/PromptExecution/just-mcp/pull/5
- Project's Conventional Commits config: `cog.toml`
- CI workflow: `.github/workflows/ci.yml`
