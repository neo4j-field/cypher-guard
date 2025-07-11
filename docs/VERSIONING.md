# Versioning and Release Management

This document explains how versioning and releases work in Cypher Guard.

## Version Format

Cypher Guard uses [Semantic Versioning](https://semver.org/) (SemVer) with the format `MAJOR.MINOR.PATCH`:

- **MAJOR**: Breaking changes that require migration
- **MINOR**: New features that are backward compatible
- **PATCH**: Bug fixes and minor improvements

## Current Version

The current version is defined in the workspace `Cargo.toml`:

```toml
[workspace.package]
version = "0.1.0"
```

## Automated Release Process

### 1. Version Bumping

Versions are automatically bumped when:

- **PRs are merged to main**: The system detects the type of change based on PR labels or commit messages
- **Manual trigger**: You can manually trigger a version bump via GitHub Actions

#### Automatic Detection Rules

- **Major**: PR contains "breaking" or "major" in title/body
- **Minor**: PR contains "feature" or "minor" in title/body  
- **Patch**: Default for all other changes

#### Manual Version Bump

1. Go to the [Actions tab](https://github.com/neo4j-field/cypher-guard/actions)
2. Select "Version Bump" workflow
3. Click "Run workflow"
4. Choose the version type (patch/minor/major)
5. Click "Run workflow"

### 2. Release Creation

When a version is bumped:

1. A new git tag is created (e.g., `v0.1.1`)
2. The tag triggers the release workflow
3. All packages are built (Rust, Python, JavaScript)
4. A GitHub release is created with:
   - Release notes
   - Compiled binaries
   - Package files
   - Checksums for verification

## Local Development

### Manual Version Bump

You can bump versions locally using the provided script:

```bash
# Bump patch version (0.1.0 -> 0.1.1)
./scripts/bump-version.sh patch

# Bump minor version (0.1.0 -> 0.2.0)
./scripts/bump-version.sh minor

# Bump major version (0.1.0 -> 1.0.0)
./scripts/bump-version.sh major
```

The script will:
- Update all `Cargo.toml` files
- Update Python `pyproject.toml` (if exists)
- Update JavaScript `package.json` (if exists)
- Create a git commit
- Create a git tag

### Checking Current Version

```bash
# From workspace root
grep '^version = ' Cargo.toml

# Or use the script to see what would happen
./scripts/bump-version.sh patch --dry-run
```

## Release Assets

Each release includes:

### Rust
- `cypher_guard-x86_64-unknown-linux-gnu` - Linux binary
- `libcypher_guard-linux.so` - Linux shared library

### Python
- `cypher_guard-*.whl` - Python wheel package

### JavaScript
- `cypher_guard-*.tgz` - npm package

### Tools
- `cypher_guard_eval-x86_64-unknown-linux-gnu` - Evaluation tool
- `checksums.txt` - SHA256 checksums for all files

## Workflow Files

- `.github/workflows/release.yml` - Creates releases when tags are pushed
- `.github/workflows/version-bump.yml` - Handles version bumping
- `scripts/bump-version.sh` - Local version bump script

## Best Practices

### For Contributors

1. **Use conventional commit messages** to help with automatic version detection
2. **Add appropriate PR labels** (breaking, feature, bugfix)
3. **Update documentation** when making breaking changes

### For Maintainers

1. **Review PRs carefully** before merging to main
2. **Use manual version bumps** for important releases
3. **Verify release assets** after each release
4. **Update release notes** with meaningful descriptions

### Commit Message Format

```
type(scope): description

[optional body]

[optional footer]
```

Types that affect versioning:
- `feat:` - Minor version bump
- `fix:` - Patch version bump  
- `BREAKING CHANGE:` - Major version bump

## Troubleshooting

### Release Failed

1. Check the [Actions tab](https://github.com/neo4j-field/cypher-guard/actions) for error details
2. Verify all builds pass locally
3. Check that version numbers are consistent across all packages

### Version Mismatch

If versions get out of sync:

```bash
# Reset to a known good state
git checkout main
git pull origin main

# Manually bump to desired version
./scripts/bump-version.sh patch
```

### Manual Release

If automated release fails, you can create a release manually:

1. Create and push a tag: `git tag v0.1.1 && git push origin v0.1.1`
2. Go to [Releases page](https://github.com/neo4j-field/cypher-guard/releases)
3. Click "Draft a new release"
4. Select the tag and fill in release notes
5. Upload built assets manually 