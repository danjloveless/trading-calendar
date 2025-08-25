# trading-calendar 📅

[![Crates.io](https://img.shields.io/crates/v/trading-calendar.svg)](https://crates.io/crates/trading-calendar)
[![Documentation](https://docs.rs/trading-calendar/badge.svg)](https://docs.rs/trading-calendar)
[![License](https://img.shields.io/crates/l/trading-calendar.svg)](https://github.com/danjloveless/trading-calendar#license)
[![CI](https://github.com/danjloveless/trading-calendar/workflows/CI/badge.svg)](https://github.com/danjloveless/trading-calendar/actions)

A comprehensive trading calendar for global financial markets, providing holidays, trading hours, and early close information.

## Features

- 🌍 **Multiple Markets**: NYSE, NASDAQ, LSE, TSE, TSX
- ⏰ **Trading Hours**: Regular, pre-market, and after-hours sessions
- 📅 **Holiday Detection**: All market holidays with weekend adjustments
- 🕐 **Early Closes**: Half-day schedules (Christmas Eve, Black Friday, etc.)
- 🌐 **Timezone Support**: Automatic handling of market timezones
- 🚀 **Performance**: Efficient LRU caching
- 🔒 **Thread Safe**: Concurrent access support
- 📆 **2020-2030 Support**: Comprehensive holiday calendars

## Quick Start

```rust
use trading_calendar::{TradingCalendar, Market};

fn main() -> trading_calendar::Result<()> {
    let nyse = TradingCalendar::new(Market::NYSE)?;
    
    // Check if market is open
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

## Supported Markets

| Market | Regular Hours (Local) | Pre-Market | After-Hours | Status |
|--------|----------------------|------------|-------------|---------|
| NYSE | 9:30 AM - 4:00 PM ET | 4:00 AM - 9:30 AM | 4:00 PM - 8:00 PM | ✅ Full Support |
| NASDAQ | 9:30 AM - 4:00 PM ET | 4:00 AM - 9:30 AM | 4:00 PM - 8:00 PM | ✅ Full Support |
| LSE | 8:00 AM - 4:30 PM GMT | - | - | ✅ Full Support |
| TSE | 9:00 AM - 3:00 PM JST | - | - | ✅ Full Support |
| TSX | 9:30 AM - 4:00 PM ET | - | - | ✅ Full Support |

## Thread Safety

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
    });
    
    Ok(())
}
```

## Error Handling

The library uses proper error handling with `Result` types:

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

## Performance

The library uses efficient caching to ensure optimal performance:

- Holiday calculations are cached per year using LRU cache
- Thread-safe concurrent access with proper eviction
- Minimal allocations with optimized data structures

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
trading-calendar = "0.2.0"
```

## Key Methods

```rust
use trading_calendar::{TradingCalendar, Market};

let calendar = TradingCalendar::new(Market::NYSE)?;

// Check if market is open now
let is_open = calendar.is_open_now()?;

// Check if a specific date is a trading day
let is_trading = calendar.is_trading_day(date)?;

// Check if a specific date is a holiday
let is_holiday = calendar.is_holiday(date)?;

// Get next market open time
let next_open = calendar.next_open()?;

// Get next market close time
let next_close = calendar.next_close()?;

// Get trading hours for a specific date
let hours = calendar.trading_hours(date);
```

## Examples

See the [examples](./examples/) directory for more detailed usage examples:

- [Basic Usage](./examples/basic_usage.rs) - Simple calendar operations
- [Check Holidays](./examples/check_holidays.rs) - Holiday detection
- [Holiday Info](./examples/holiday_info.rs) - Detailed holiday information

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.