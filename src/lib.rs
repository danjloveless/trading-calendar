//! # Trading Calendar
//!
//! A comprehensive trading calendar for global financial markets, providing holidays,
//! trading hours, and early close information.
//!
//! ## Features
//!
//! - 🌍 **Multiple Markets**: NYSE, NASDAQ, LSE, TSE, TSX
//! - ⏰ **Trading Hours**: Regular, pre-market, and after-hours sessions
//! - 📅 **Holiday Detection**: All market holidays with weekend adjustments
//! - 🕐 **Early Closes**: Half-day schedules (Christmas Eve, Black Friday, etc.)
//! - 🌐 **Timezone Support**: Automatic handling of market timezones
//! - 🚀 **Performance**: Efficient LRU caching
//! - 🔒 **Thread Safe**: Concurrent access support
//! - 📆 **2020-2030 Support**: Comprehensive holiday calendars
//!
//! ## Quick Start
//!
//! ```rust
//! use trading_calendar::{TradingCalendar, Market};
//!
//! fn main() -> trading_calendar::Result<()> {
//!     let nyse = TradingCalendar::new(Market::NYSE)?;
//!     
//!     // Check if market is open
//!     if nyse.is_open_now()? {
//!         println!("NYSE is open for trading!");
//!     }
//!     
//!     // Get next market open
//!     let next_open = nyse.next_open()?;
//!     println!("NYSE opens: {}", next_open);
//!     
//!     // Check specific date
//!     let christmas = chrono::NaiveDate::from_ymd_opt(2025, 12, 25).unwrap();
//!     if !nyse.is_trading_day(christmas)? {
//!         println!("Market closed on Christmas");
//!     }
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## Supported Markets
//!
//! | Market | Regular Hours (Local) | Pre-Market | After-Hours | Status |
//! |--------|----------------------|------------|-------------|---------|
//! | NYSE | 9:30 AM - 4:00 PM ET | 4:00 AM - 9:30 AM | 4:00 PM - 8:00 PM | ✅ Full Support |
//! | NASDAQ | 9:30 AM - 4:00 PM ET | 4:00 AM - 9:30 AM | 4:00 PM - 8:00 PM | ✅ Full Support |
//! | LSE | 8:00 AM - 4:30 PM GMT | - | - | ✅ Full Support |
//! | TSE | 9:00 AM - 3:00 PM JST | - | - | ✅ Full Support |
//! | TSX | 9:30 AM - 4:00 PM ET | - | - | ✅ Full Support |
//!
//! ## Thread Safety
//!
//! The `TradingCalendar` is thread-safe and can be shared across threads:
//!
//! ```rust
//! use std::sync::Arc;
//! use trading_calendar::{TradingCalendar, Market};
//!
//! fn main() -> trading_calendar::Result<()> {
//!     let calendar = Arc::new(TradingCalendar::new(Market::NYSE)?);
//!
//!     // Share calendar across threads safely
//!     let cal_clone = Arc::clone(&calendar);
//!     std::thread::spawn(move || {
//!         let is_open = cal_clone.is_open_now().unwrap_or(false);
//!     });
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## Error Handling
//!
//! The library uses proper error handling with `Result` types:
//!
//! ```rust
//! use trading_calendar::{TradingCalendar, Market, CalendarError};
//!
//! fn main() -> trading_calendar::Result<()> {
//!     let calendar = TradingCalendar::new(Market::NYSE)?;
//!
//!     // Check for unsupported years
//!     match calendar.is_trading_day(chrono::NaiveDate::from_ymd_opt(2019, 1, 1).unwrap()) {
//!         Ok(is_trading) => println!("Is trading day: {}", is_trading),
//!         Err(CalendarError::DateOutOfRange(date)) => println!("Date {} not supported", date),
//!         Err(e) => eprintln!("Error: {}", e),
//!     }
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## Performance
//!
//! The library uses efficient caching to ensure optimal performance:
//!
//! - Holiday calculations are cached per year using LRU cache
//! - Thread-safe concurrent access with proper eviction
//! - Minimal allocations with optimized data structures
//!
//! ## License
//!
//! Licensed under either of:
//!
//! - Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
//! - MIT license ([LICENSE-MIT](LICENSE-MIT))
//!
//! at your option.

#![deny(unsafe_code)]
#![forbid(unsafe_code)]

pub mod calendar;
pub mod constants;
pub mod error;
pub mod markets;
pub mod schedule;
pub mod utils;

// Re-export main types
pub use calendar::TradingCalendar;
pub use error::{CalendarError, Result};
pub use markets::Market;
pub use schedule::{Session, TradingHours};

// Re-export chrono types for convenience
pub use chrono::{DateTime, NaiveDate, NaiveTime, Utc};

// Global constants
/// Minimum supported year
pub const MIN_YEAR: i32 = 2020;
/// Maximum supported year
pub const MAX_YEAR: i32 = 2030;

/// Holiday information
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Holiday {
    /// The date of the holiday
    pub date: NaiveDate,
    /// The name of the holiday
    pub name: String,
    /// Whether the market is completely closed
    pub market_closed: bool,
    /// Early close time if applicable
    pub early_close: Option<NaiveTime>,
}

impl Holiday {
    /// Create a new holiday
    pub fn new(date: NaiveDate, name: &str, market_closed: bool) -> Self {
        Holiday {
            date,
            name: name.to_string(),
            market_closed,
            early_close: None,
        }
    }

    /// Create a holiday with early close
    pub fn with_early_close(date: NaiveDate, name: &str, early_close: NaiveTime) -> Self {
        Holiday {
            date,
            name: name.to_string(),
            market_closed: false,
            early_close: Some(early_close),
        }
    }
}
