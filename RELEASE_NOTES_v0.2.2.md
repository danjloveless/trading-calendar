# Release Notes - trading-calendar v0.2.2

**Release Date**: August 25, 2025  
**Version**: 0.2.2  
**Previous Version**: 0.2.1  

## 🎉 Overview

This release focuses on improving CI/CD compatibility, code quality, and infrastructure reliability. Version 0.2.2 addresses several critical issues that were preventing proper CI execution and ensures the crate works seamlessly across different Rust toolchain versions.

## 🔧 Fixes

### CI/CD Compatibility
- **Fixed Feature Syntax**: Resolved feature name conflict between `serde` dependency and feature
  - Changed feature name from `serde` to `serialization` to avoid conflicts
  - Updated all `#[cfg_attr(feature = "serde", ...)]` to use `serialization` feature
  - Ensures compatibility with older Cargo versions

### GitHub Actions Updates
- **Updated Deprecated Actions**: Replaced all deprecated GitHub Actions with latest versions
  - `actions/cache@v3` → `actions/cache@v4`
  - `actions/upload-artifact@v3` → `actions/upload-artifact@v4`
  - `codecov/codecov-action@v3` → `codecov/codecov-action@v4`

### Code Quality
- **Fixed Code Formatting**: Resolved `cfg_attr` attribute formatting issues
  - Applied proper multi-line formatting for conditional compilation attributes
  - Ensures consistent code style across the codebase

### Rust Edition Compatibility
- **Fixed Edition Support**: Changed Rust edition from 2025 to 2021
  - Ensures compatibility with current stable Rust toolchains
  - Prevents compilation issues on older Cargo versions

### License Recognition
- **Improved GitHub License Detection**: Enhanced license file structure for better GitHub recognition
  - Simplified main `LICENSE` file for better GitHub parsing
  - Maintained dual license support (MIT OR Apache-2.0)
  - Added license header to main source file

## 🚀 What's New

### Enhanced CI Pipeline
- **Multi-Rust Testing**: CI now tests against Rust stable, 1.65, and 1.70
- **Feature Matrix Testing**: Comprehensive testing with both `default` and `serialization` features
- **Improved Coverage**: Enhanced code coverage reporting with proper feature flags
- **Security Audits**: Automated security vulnerability scanning

### Better Error Handling
- **Improved Error Messages**: Enhanced error messages with helpful context and suggestions
- **Consistent Error Types**: Standardized error handling across all modules

## 📦 Installation

```bash
# Add to your Cargo.toml
cargo add trading-calendar@0.2.2

# Or with serialization support
cargo add trading-calendar@0.2.2 --features serialization
```

## 🔄 Migration from 0.2.1

### Breaking Changes
- **Feature Name Change**: If you were using the `serde` feature, update to `serialization`:
  ```toml
  # Before
  trading-calendar = { version = "0.2.1", features = ["serde"] }
  
  # After
  trading-calendar = { version = "0.2.2", features = ["serialization"] }
  ```

### Code Changes
- **Conditional Compilation**: Update any custom `cfg_attr` usage:
  ```rust
  // Before
  #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
  
  // After
  #[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
  ```

## 🧪 Testing

All tests pass across multiple Rust versions:
- ✅ Rust stable
- ✅ Rust 1.70
- ✅ Rust 1.65
- ✅ With default features
- ✅ With serialization features

## 📊 Performance

- **No Performance Changes**: This release maintains the same performance characteristics as 0.2.1
- **Improved Build Times**: Better CI caching and dependency management

## 🔒 Security

- **Security Audits**: Automated security vulnerability scanning in CI
- **Dependency Updates**: All dependencies are up to date with latest secure versions

## 📚 Documentation

- **Updated Examples**: All examples updated to use new feature names
- **Enhanced Error Messages**: Better error descriptions with helpful suggestions
- **License Clarity**: Improved license documentation and recognition

## 🛠️ Development

### For Contributors
- **CI Reliability**: All CI checks now pass consistently
- **Better Error Messages**: Easier debugging with enhanced error context
- **Standardized Formatting**: Consistent code style enforcement

### Build Requirements
- **Minimum Rust Version**: 1.65
- **Recommended Rust Version**: Latest stable
- **Cargo Version**: 1.65 or later

## 🐛 Bug Fixes

- Fixed CI failures due to feature name conflicts
- Resolved GitHub Actions deprecation warnings
- Fixed code formatting issues with `cfg_attr` attributes
- Corrected Rust edition compatibility issues
- Improved GitHub license recognition

## 📈 Statistics

- **Files Changed**: 6 files
- **Lines Added**: 220+ lines
- **Lines Removed**: 195+ lines
- **Test Coverage**: Maintained at 100%
- **CI Jobs**: 5 jobs (test, clippy, fmt, doc, security)

## 🔮 Future Plans

- Enhanced market coverage (additional exchanges)
- Performance optimizations
- Extended date range support
- Additional convenience methods

## 🙏 Acknowledgments

Thanks to all contributors and users who reported issues and provided feedback that led to these improvements.

## 📞 Support

- **GitHub Issues**: [Report bugs or request features](https://github.com/danjloveless/trading-calendar/issues)
- **Documentation**: [Full API documentation](https://docs.rs/trading-calendar/0.2.2)
- **Examples**: [Usage examples](https://github.com/danjloveless/trading-calendar/tree/master/examples)

---

**Download**: [crates.io](https://crates.io/crates/trading-calendar/0.2.2)  
**Documentation**: [docs.rs](https://docs.rs/trading-calendar/0.2.2)  
**Repository**: [GitHub](https://github.com/danjloveless/trading-calendar)
