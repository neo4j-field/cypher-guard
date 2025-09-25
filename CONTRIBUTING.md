# Contributing to Cypher Guard

## Pull Request Process

1. **Use the PR Template**: When creating a pull request, please use the provided template that includes:
   - Description of changes
   - Type of change (feature, bug fix, etc.)
   - Testing information
   - Checklist items

2. **Required Information**: All PRs must include:
   - Clear description of what was changed
   - Type of change (check the appropriate boxes)
   - How the changes were tested
   - Updated documentation if applicable

3. **Release Notes**: If your changes should be included in release notes, add a "## Release Notes" section to your PR description.

## Development Setup

See [docs/DEVELOPMENT.md](docs/DEVELOPMENT.md) for detailed setup instructions.

## Code Style

- Run `cargo fmt --all` before committing
- Run `cargo clippy -- -D warnings` to check for linting issues
- Ensure all tests pass with `cargo test`
