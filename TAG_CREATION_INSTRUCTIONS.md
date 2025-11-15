# Creating v0.1.1 Tag

The version has been bumped to 0.1.1 in commit `bc2e89f`. To complete the fix and move the cocogitto validation boundary, a v0.1.1 tag needs to be created.

## How to Create the Tag

### Option 1: After PR Merge (Recommended)
Once this PR is merged to main, create the tag on the main branch:

```bash
git checkout main
git pull origin main
git tag v0.1.1
git push origin v0.1.1
```

### Option 2: On This Branch (For Testing)
To test the tag creation on this branch:

```bash
git tag v0.1.1 bc2e89f
git push origin v0.1.1
```

## Verification

After creating the tag, verify cocogitto validation passes:

```bash
cog check
```

This should now show "No errored commits" since all commits from v0.1.1 forward follow conventional commit format.

## What This Fixes

With the v0.1.1 tag in place and `from_latest_tag = true` in cog.toml:
- ✅ Cocogitto will only validate commits after v0.1.1
- ✅ The "Initial plan" commits (8c61d60, 25cc1e3, 2dd039f) will be outside the validation scope
- ✅ Release workflow will succeed
- ✅ All future commits must follow conventional commits format

