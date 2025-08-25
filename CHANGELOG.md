# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2024-12-19

### Added
- **Cache Performance Improvements**: Added `get()`, `insert()`, `clear()`, and `stats()` methods to `HolidayCache`
- **Cache Statistics**: New `CacheStats` struct for monitoring cache performance
- **Enhanced Documentation**: Added comprehensive doc comments and examples
- **Better Error Handling**: Improved error messages and validation
- **Comprehensive Test Suite**: Added tests for cache operations and edge cases

### Changed
- **BREAKING**: `is_trading_day()` and `is_holiday()` now return `Result<bool>` instead of `bool`
- **BREAKING**: `is_open_now()` now returns `Result<bool>` instead of `bool`
- **Memory Safety**: Replaced panic-prone constants with safe `unwrap()` calls
- **Version Bump**: Updated to 0.2.0 to reflect breaking changes
- **Documentation**: Added `[package.metadata.docs.rs]` configuration for better docs.rs integration

### Fixed
- **Critical**: Juneteenth holiday now only applies from 2021 onwards (was incorrectly applied to all years)
- **Critical**: After-hours trading now works correctly on early close days
- **Critical**: Year range validation now returns proper errors instead of silent false
- **Critical**: UK Boxing Day logic corrected for Christmas on Saturday
- **Critical**: Japan Golden Week May 4 substitute holiday logic completed
- **Major**: DST ambiguity handling improved
- **Major**: `next_close()` now properly handles year boundaries
- **Performance**: Improved cache operations with better API
- **Code Quality**: Replaced magic numbers with named constants
- **Dependencies**: Pinned thiserror to exact version 2.0.0

### Security
- **Memory Safety**: Removed panic-prone code from constants module
- **Thread Safety**: Improved cache implementation with better concurrency handling

## [0.1.0] - 2024-12-17

### Added
- **Initial Release**: Basic trading calendar functionality
- **US Markets**: NYSE and NASDAQ support
- **Core Features**: Trading day detection and holiday checking
- **Documentation**: Basic README and examples

---

## Migration Guide

### From 0.1.x to 0.2.0

**Breaking Changes:**

The following methods now return `Result<bool>` instead of `bool`:

**Before:**
```rust
let calendar = TradingCalendar::new(Market::NYSE)?;

// These returned bool
let is_trading = calendar.is_trading_day(date);
let is_holiday = calendar.is_holiday(date);
let is_open = calendar.is_open_now();
```

**After:**
```rust
let calendar = TradingCalendar::new(Market::NYSE)?;

// These now return Result<bool>
let is_trading = calendar.is_trading_day(date)?;
let is_holiday = calendar.is_holiday(date)?;
let is_open = calendar.is_open_now()?;
```

**Error Handling:**
- Dates outside the supported range (2020-2030) now return `CalendarError::DateOutOfRange`
- Invalid timezone conversions now return `CalendarError::InvalidTime`

**Cache API Improvements:**
```rust
// New methods available
let cache = HolidayCache::new();
cache.insert(2025, holidays);
let cached = cache.get(2025);
let stats = cache.stats();
cache.clear();
```
