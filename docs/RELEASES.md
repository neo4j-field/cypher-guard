# Releases

Cypher Guard uses automated releases with semantic versioning. Releases are created automatically when code is merged to main.

## Current Version

**v0.1.0** - Initial release with core validation functionality

## Release Process

1. **Automatic Version Bumping**: When PRs are merged to main, versions are automatically bumped based on:
   - **Major**: Breaking changes (PR contains "breaking" or "major")
   - **Minor**: New features (PR contains "feature" or "minor")  
   - **Patch**: Bug fixes (default for all other changes)

2. **Release Creation**: Each version bump triggers:
   - Git tag creation (e.g., `v0.1.1`)
   - Automated builds for all platforms
   - GitHub release with compiled binaries and packages
   - Release notes with changelog

## Download

Get the latest release from [GitHub Releases](https://github.com/neo4j-field/cypher-guard/releases):

- **Python**: `cypher_guard-*.whl` - Install with `pip install cypher_guard-*.whl`
- **JavaScript**: `cypher_guard-*.tgz` - Install with `npm install cypher_guard-*.tgz`
- **Rust**: Add to `Cargo.toml` with `cargo add cypher-guard`
- **Evaluation Tool**: `cypher_guard_eval-*` - Standalone evaluation suite

## Manual Release

For manual releases or version bumps:

```bash
# Bump version locally
./scripts/bump-version.sh patch  # or minor/major

# Or trigger via GitHub Actions
# Go to Actions → Version Bump → Run workflow
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

## Installation Examples

### Python
```bash
# Download the wheel file from releases
pip install cypher_guard-0.1.0-cp311-cp311-linux_x86_64.whl

# Or install from source
pip install git+https://github.com/neo4j-field/cypher-guard.git
```

### JavaScript
```bash
# Download the tgz file from releases
npm install cypher_guard-0.1.0.tgz

# Or install from source
npm install git+https://github.com/neo4j-field/cypher-guard.git
```

### Rust
```toml
# Add to Cargo.toml
[dependencies]
cypher-guard = "0.1.0"
```

## Changelog

### v0.1.0 (Initial Release)
- Core Cypher query validation functionality
- Support for MATCH, RETURN, WHERE, CREATE, MERGE clauses
- Node and relationship pattern validation
- Property access and type checking
- Python and JavaScript bindings
- Comprehensive error reporting with structured error types
- Evaluation suite for testing validation accuracy

## Related Documentation

- [Versioning Guide](VERSIONING.md) - Detailed version management documentation
- [API Documentation](API.md) - Complete API reference
- [Schema Format](SCHEMA.md) - Schema definition guide 