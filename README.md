# trading-calendar 📅

[![Crates.io](https://img.shields.io/crates/v/trading-calendar.svg)](https://crates.io/crates/trading-calendar)
[![Documentation](https://docs.rs/trading-calendar/badge.svg)](https://docs.rs/trading-calendar)
[![License](https://img.shields.io/crates/l/trading-calendar.svg)](https://github.com/danjloveless/trading-calendar#license)
[![CI](https://github.com/danjloveless/trading-calendar/workflows/CI/badge.svg)](https://github.com/danjloveless/trading-calendar/actions)
[![Rust Version](https://img.shields.io/badge/rust-1.75+-blue.svg)](https://www.rust-lang.org)

A comprehensive trading calendar for global financial markets, providing holidays, trading hours, and early close information. Built with performance and reliability in mind, this library supports major exchanges worldwide with accurate holiday calculations and timezone handling.

## 📚 Documentation

- **[API Documentation](https://docs.rs/trading-calendar)** - Complete API reference
- **[Examples](./examples/)** - Practical usage examples and tutorials
- **[Changelog](./CHANGELOG.md)** - Version history and changes
- **[Contributing](./CONTRIBUTING.md)** - How to contribute to the project
- **[Release Notes v0.2.3](./RELEASE_NOTES_v0.2.3.md)** - Detailed release information

## ✨ Features

- 🌍 **Multiple Markets**: NYSE, NASDAQ, LSE, TSE, TSX with accurate holiday calendars
- ⏰ **Trading Hours**: Regular, pre-market, and after-hours sessions with timezone support
- 📅 **Holiday Detection**: All market holidays with weekend adjustments and early closes
- 🕐 **Early Closes**: Half-day schedules (Christmas Eve, Black Friday, etc.)
- 🌐 **Timezone Support**: Automatic handling of market timezones (ET, GMT, JST)
- 🚀 **Performance**: Efficient LRU caching for holiday calculations
- 🔒 **Thread Safe**: Concurrent access support with proper synchronization
- 📆 **Date Range**: Comprehensive support for years 2020-2030
- 🔧 **Error Handling**: Robust error handling with detailed error messages
- 📦 **Serialization**: Optional serde support for JSON serialization

## 🚀 Quick Start

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
trading-calendar = "0.2.3"

# With serialization support
trading-calendar = { version = "0.2.3", features = ["serialization"] }
```

### Basic Usage

```rust
use trading_calendar::{TradingCalendar, Market};

fn main() -> trading_calendar::Result<()> {
    let nyse = TradingCalendar::new(Market::NYSE)?;
    
    // Check if market is open now
    if nyse.is_open_now()? {
        println!("NYSE is open for trading!");
    }
    
    // Get next market open
    let next_open = nyse.next_open()?;
    println!("NYSE opens: {}", next_open);
    
    // Check specific date
    let christmas = chrono::NaiveDate::from_ymd_opt(2025, 12, 25).unwrap();
    if !nyse.is_trading_day(christmas)? {
        println!("Market closed on Christmas");
    }
    
    Ok(())
}
```

## 📊 Supported Markets

| Market | Regular Hours (Local) | Pre-Market | After-Hours | Timezone | Status |
|--------|----------------------|------------|-------------|----------|---------|
| NYSE | 9:30 AM - 4:00 PM ET | 4:00 AM - 9:30 AM | 4:00 PM - 8:00 PM | ET | ✅ Full Support |
| NASDAQ | 9:30 AM - 4:00 PM ET | 4:00 AM - 9:30 AM | 4:00 PM - 8:00 PM | ET | ✅ Full Support |
| LSE | 8:00 AM - 4:30 PM GMT | - | - | GMT | ✅ Full Support |
| TSE | 9:00 AM - 3:00 PM JST | - | - | JST | ✅ Full Support |
| TSX | 9:30 AM - 4:00 PM ET | - | - | ET | ✅ Full Support |

## 🔧 API Reference

### Core Methods

```rust
use trading_calendar::{TradingCalendar, Market};

let calendar = TradingCalendar::new(Market::NYSE)?;

// Market status
let is_open = calendar.is_open_now()?;
let is_trading = calendar.is_trading_day(date)?;
let is_holiday = calendar.is_holiday(date)?;

// Time navigation
let next_open = calendar.next_open()?;
let next_close = calendar.next_close()?;
let next_trading_day = calendar.next_trading_day(date)?;
let prev_trading_day = calendar.prev_trading_day(date)?;

// Trading hours
let hours = calendar.trading_hours(date);
let is_early_close = calendar.is_early_close(date)?;

// Utility methods
let trading_days = calendar.trading_days_in_month(year, month)?;
let count = calendar.count_trading_days(start_date, end_date)?;
```

### Error Handling

```rust
use trading_calendar::{TradingCalendar, Market, CalendarError};

fn main() -> trading_calendar::Result<()> {
    let calendar = TradingCalendar::new(Market::NYSE)?;

    // Check for unsupported years
    match calendar.is_trading_day(chrono::NaiveDate::from_ymd_opt(2019, 1, 1).unwrap()) {
        Ok(is_trading) => println!("Is trading day: {}", is_trading),
        Err(CalendarError::DateOutOfRange(date)) => println!("Date {} not supported", date),
        Err(e) => eprintln!("Error: {}", e),
    }
    
    Ok(())
}
```

## 🔄 Thread Safety

The `TradingCalendar` is thread-safe and can be shared across threads:

```rust
use std::sync::Arc;
use trading_calendar::{TradingCalendar, Market};

fn main() -> trading_calendar::Result<()> {
    let calendar = Arc::new(TradingCalendar::new(Market::NYSE)?);

    // Share calendar across threads safely
    let cal_clone = Arc::clone(&calendar);
    std::thread::spawn(move || {
        let is_open = cal_clone.is_open_now().unwrap_or(false);
        println!("Market open: {}", is_open);
    });
    
    Ok(())
}
```

## 📦 Serialization Support

Enable serialization features for JSON support:

```rust
use trading_calendar::{TradingCalendar, Market};

// In Cargo.toml: trading-calendar = { version = "0.2.3", features = ["serialization"] }

let calendar = TradingCalendar::new(Market::NYSE)?;
let json = serde_json::to_string(&calendar)?;
println!("Calendar JSON: {}", json);
```

## 📖 Examples

See the [examples directory](./examples/) for detailed usage examples:

- **[Basic Usage](./examples/basic_usage.rs)** - Simple calendar operations and market status checks
- **[Check Holidays](./examples/check_holidays.rs)** - Holiday detection and listing
- **[Holiday Info](./examples/holiday_info.rs)** - Detailed holiday information and early close handling

### Running Examples

```bash
# Run a specific example
cargo run --example basic_usage

# Run all examples
for example in basic_usage check_holidays holiday_info; do
    cargo run --example $example
done
```

## 🧪 Testing

Run the comprehensive test suite:

```bash
# All tests
cargo test --all-features

# Specific test categories
cargo test --test integration_tests
cargo test --test market_tests
cargo test --test edge_cases
```

## 📈 Performance

The library is optimized for performance:

- **LRU Caching**: Holiday calculations are cached per year
- **Thread-Safe**: Concurrent access with proper eviction
- **Minimal Allocations**: Optimized data structures
- **Benchmarks**: Performance benchmarks available in `benches/`

## 🔒 Security

- **Security Audits**: Automated vulnerability scanning in CI
- **Dependency Updates**: Regular security updates
- **Safe by Default**: Memory-safe Rust implementation

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](./CONTRIBUTING.md) for:

- Code style guidelines
- Testing requirements
- Documentation standards
- Pull request process
- Holiday rule verification

## 📄 License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](./LICENSE-APACHE))
- MIT license ([LICENSE-MIT](./LICENSE-MIT))

at your option.

## 📞 Support

- **GitHub Issues**: [Report bugs or request features](https://github.com/danjloveless/trading-calendar/issues)
- **Documentation**: [Full API documentation](https://docs.rs/trading-calendar)
- **Examples**: [Usage examples](./examples/)
- **Changelog**: [Version history](./CHANGELOG.md)

## 🔗 Links

- **Crates.io**: [trading-calendar](https://crates.io/crates/trading-calendar)
- **Documentation**: [docs.rs](https://docs.rs/trading-calendar)
- **Repository**: [GitHub](https://github.com/danjloveless/trading-calendar)
- **CI/CD**: [GitHub Actions](https://github.com/danjloveless/trading-calendar/actions)

---

**Current Version**: 0.2.3  
**Minimum Rust Version**: 1.75  
**License**: MIT OR Apache-2.0