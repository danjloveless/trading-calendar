# Release Notes - trading-calendar v0.2.3

**Date**: August 25, 2025  
**Version**: 0.2.3  
**Previous Version**: 0.2.2

## 🚀 Overview

This release focuses on resolving critical CI/CD issues and dependency compatibility problems. Version 0.2.3 addresses network connectivity issues in GitHub Actions and resolves dependency version conflicts that were preventing successful builds across different Rust toolchain versions.

## 🔧 Key Changes

### CI/CD Improvements
- **Network Reliability**: Replaced `actions-rs/toolchain@v1` with `dtolnay/rust-toolchain@stable` for better network resilience
- **Backup Workflow**: Added manual rustup installation with retry logic as fallback
- **Workflow Consistency**: Updated all GitHub Actions workflows to use reliable toolchain installation

### Dependency Compatibility
- **Pinned Dependencies**: Pinned `criterion` to version 0.3.3 and `rayon` to version 1.4.0
- **Version Conflicts**: Resolved `clap_lex` and `rayon-core` version conflicts
- **Minimum Rust Version**: Updated from 1.65 to 1.75 to ensure compatibility

### Technical Details
- **CI Matrix**: Updated to test with Rust 1.75, 1.80, and stable
- **Dependency Resolution**: Fixed issues with newer dependency versions requiring Rust 1.80+
- **Build Reliability**: Ensured consistent builds across different environments

## 📦 Installation

### Basic Installation
```bash
cargo add trading-calendar@0.2.3
```

### With Serialization Support
```bash
cargo add trading-calendar@0.2.3 --features serialization
```

### Manual Cargo.toml
```toml
[dependencies]
trading-calendar = { version = "0.2.3", features = ["serialization"] }
```

## 🔄 Migration from v0.2.2

### Breaking Changes
- **Minimum Rust Version**: Now requires Rust 1.75+ (previously 1.65+)
- **Dependency Versions**: Some internal dependencies have been pinned to specific versions

### Compatibility
- **API Compatibility**: No breaking changes to the public API
- **Feature Compatibility**: All existing features continue to work as expected
- **Performance**: No performance regressions

## 🧪 Testing

All tests pass across the supported Rust versions:
- ✅ Rust 1.75
- ✅ Rust 1.80  
- ✅ Rust stable

## 📋 Requirements

- **Rust**: 1.75 or higher
- **Platforms**: All platforms supported by Rust
- **Features**: All existing features remain available

## 🔗 Links

- **Documentation**: [Full API documentation](https://docs.rs/trading-calendar/0.2.3)
- **Repository**: [GitHub](https://github.com/danjloveless/trading-calendar)
- **Issues**: [GitHub Issues](https://github.com/danjloveless/trading-calendar/issues)

## 📥 Download

**Download**: [crates.io](https://crates.io/crates/trading-calendar/0.2.3)  
**Documentation**: [docs.rs](https://docs.rs/trading-calendar/0.2.3)

---

For detailed information about all changes, see the [CHANGELOG.md](./CHANGELOG.md).
