# Contributing to trading-calendar

We welcome contributions! Please follow these guidelines:

## Getting Started

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Code Style

- Run `cargo fmt` before committing
- Ensure `cargo clippy` passes without warnings
- Add tests for new functionality
- Update documentation as needed

## Testing

Run the full test suite with:

```bash
cargo test --all-features
```

### Test Categories

- **Unit Tests**: Test individual functions and methods
- **Integration Tests**: Test the public API
- **Edge Cases**: Test boundary conditions and error cases
- **Market Tests**: Test market-specific holiday calculations
- **Negative Tests**: Test error handling and invalid inputs

## Documentation

- Update doc comments for any new public APIs
- Add examples in doc comments where appropriate
- Update README.md if adding new features
- Update CHANGELOG.md for any user-facing changes

## Holiday Rules

When adding new markets or updating holiday rules:

1. Verify holiday dates with official exchange sources
2. Include weekend adjustment logic
3. Add comprehensive tests for the new market
4. Update the market comparison table in README.md

## Performance

- Profile any performance-critical changes
- Ensure cache usage is optimal
- Add benchmarks for new features if applicable

## Pull Request Guidelines

1. **Title**: Use a clear, descriptive title
2. **Description**: Explain what the PR does and why
3. **Tests**: Include tests for new functionality
4. **Documentation**: Update docs for any API changes
5. **Breaking Changes**: Clearly mark any breaking changes

## Release Process

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md` with new version
3. Create git tag: `git tag -a v0.x.0 -m "Release version 0.x.0"`
4. Push tag: `git push origin v0.x.0`
5. Publish to crates.io: `cargo publish`

## License

By contributing, you agree that your contributions will be licensed under the same terms as the project (MIT OR Apache-2.0).

## Questions?

If you have questions about contributing, please open an issue or reach out to the maintainers.
