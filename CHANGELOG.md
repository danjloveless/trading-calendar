# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.3] - 2025-01-27

### Fixed
- **CI Network Reliability**: Replaced actions-rs/toolchain@v1 with dtolnay/rust-toolchain@stable for better network resilience
- **Backup CI Workflow**: Added manual rustup installation with retry logic as fallback
- **Workflow Consistency**: Updated all GitHub Actions workflows to use reliable toolchain installation
- **Dependency Compatibility**: Pinned criterion to version 0.3.3 and rayon to version 1.4.0 to ensure compatibility with Rust 1.75+ (fixes clap_lex and rayon-core version conflicts)
- **Minimum Rust Version**: Updated minimum supported Rust version from 1.65 to 1.75 to resolve dependency conflicts

## [0.2.2] - 2025-08-25

### Fixed
- **CI Compatibility**: Fixed feature syntax to be compatible with older Cargo versions
- **Formatting**: Fixed code formatting for cfg_attr attributes
- **GitHub Actions**: Updated all deprecated actions to latest versions

## [0.2.1] - 2025-08-25

### Added
- **FromStr Implementation**: Added `FromStr` trait for `Market` enum
- **Convenience Methods**: Added `trading_days_in_month()` and `count_trading_days()` methods
- **Security Policy**: Added SECURITY.md for vulnerability reporting
- **Examples Documentation**: Added README.md in examples directory

### Fixed
- **Removed Duplicate Code**: Removed duplicate Holiday struct definition
- **Documentation**: Enhanced module-level documentation and added more examples

### Changed
- **Error Messages**: Improved error messages with helpful suggestions
- **README**: Renamed from lowercase to uppercase following conventions

## [0.2.0] - 2025-08-25

### Added
- **Initial Release**: First published version on crates.io
- **Multiple Markets**: Support for NYSE, NASDAQ, LSE, TSE, TSX
- **Trading Hours**: Regular, pre-market, and after-hours sessions
- **Holiday Detection**: Comprehensive holiday calendars for all supported markets
- **Early Closes**: Support for half-day schedules (Christmas Eve, Black Friday, etc.)
- **Timezone Support**: Automatic handling of market timezones
- **Performance**: Efficient LRU caching for holiday calculations
- **Thread Safety**: Safe concurrent access with Mutex-protected cache
- **Year Range**: Support for years 2020-2030
- **Documentation**: Comprehensive README with examples
- **Examples**: Three example programs demonstrating usage
- **Testing**: Extensive test suite including unit, integration, and edge case tests
- **Error Handling**: Custom error types with thiserror
- **CI/CD**: GitHub Actions workflow for automated testing

### Features
- Check if market is currently open
- Get next market open/close times
- Determine trading days vs holidays
- Query trading hours for any date
- Navigate to next/previous trading days
- Support for early close days
- Cache management for performance

[Unreleased]: https://github.com/danjloveless/trading-calendar/compare/v0.2.3...HEAD
[0.2.3]: https://github.com/danjloveless/trading-calendar/compare/v0.2.2...v0.2.3
[0.2.2]: https://github.com/danjloveless/trading-calendar/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/danjloveless/trading-calendar/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/danjloveless/trading-calendar/releases/tag/v0.2.0
