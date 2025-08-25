# Release v0.2.1

## 🎉 What's New

This release adds several improvements to make the trading calendar more user-friendly and better documented.

### ✨ New Features

- **FromStr Implementation**: Added `FromStr` trait for `Market` enum
  ```rust
  use trading_calendar::Market;
  let market: Market = "NYSE".parse()?;
  ```

- **Convenience Methods**: Added new methods for common operations
  ```rust
  // Get all trading days in a month
  let days = calendar.trading_days_in_month(2025, 1)?;
  
  // Count trading days between dates
  let count = calendar.count_trading_days(start, end)?;
  ```

### 📚 Documentation Improvements

- **Enhanced Module Documentation**: Added comprehensive documentation for all modules
- **Better API Examples**: More examples in doc comments and README
- **Examples README**: Added `examples/README.md` with usage instructions

### 🔧 Code Quality

- **Removed Duplicate Code**: Eliminated duplicate Holiday struct definition
- **Improved Error Messages**: More helpful error messages with context
- **Better Code Organization**: Cleaner module structure

### 📦 Package Metadata

- **Maintenance Badge**: Added actively-developed status
- **Better Cargo.toml**: Improved metadata for crates.io

## 🚀 Installation

```bash
cargo add trading-calendar
```

## 📖 Quick Start

```rust
use trading_calendar::{TradingCalendar, Market};

fn main() -> trading_calendar::Result<()> {
    let nyse = TradingCalendar::new(Market::NYSE)?;
    
    if nyse.is_open_now()? {
        println!("NYSE is open for trading!");
    }
    
    Ok(())
}
```

## 🔗 Links

- **Documentation**: https://docs.rs/trading-calendar
- **Crates.io**: https://crates.io/crates/trading-calendar
- **Repository**: https://github.com/danjloveless/trading-calendar

## 📋 Supported Markets

- **NYSE/NASDAQ**: US markets with pre-market and after-hours trading
- **LSE**: London Stock Exchange with UK bank holidays
- **TSE**: Tokyo Stock Exchange with Japanese national holidays
- **TSX**: Toronto Stock Exchange with Canadian holidays

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 📄 License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.
