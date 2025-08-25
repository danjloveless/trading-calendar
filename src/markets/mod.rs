//! Market definitions and implementations

use crate::{Result, TradingHours};
use chrono::{Datelike, NaiveDate, Weekday};
use chrono_tz::Tz;
use std::fmt;

pub mod canada;
pub mod japan;
pub mod uk;
pub mod us;

/// Supported financial markets
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Market {
    /// New York Stock Exchange
    NYSE,
    /// NASDAQ Stock Market  
    NASDAQ,
    /// London Stock Exchange
    LSE,
    /// Tokyo Stock Exchange
    TSE,
    /// Toronto Stock Exchange
    TSX,
}

/// Internal trait for market implementations
pub trait MarketImpl: Send + Sync {
    /// Check if a date is a holiday
    fn is_holiday(&self, date: NaiveDate) -> bool;

    /// Get trading hours for a date
    fn trading_hours(&self, date: NaiveDate) -> TradingHours;

    /// Get the timezone
    fn timezone(&self) -> Tz;

    /// Check if a date is a trading day
    fn is_trading_day(&self, date: NaiveDate) -> bool {
        !self.is_holiday(date) && !is_weekend(date)
    }

    /// Get the next trading day
    fn next_trading_day(&self, date: NaiveDate) -> NaiveDate {
        let mut next = date + chrono::Duration::days(1);
        while !self.is_trading_day(next) {
            next += chrono::Duration::days(1);
        }
        next
    }

    /// Get the previous trading day
    fn previous_trading_day(&self, date: NaiveDate) -> NaiveDate {
        let mut prev = date - chrono::Duration::days(1);
        while !self.is_trading_day(prev) {
            prev -= chrono::Duration::days(1);
        }
        prev
    }
}

impl Market {
    /// Get the timezone for this market
    pub fn timezone(&self) -> Tz {
        match self {
            Market::NYSE | Market::NASDAQ => chrono_tz::America::New_York,
            Market::LSE => chrono_tz::Europe::London,
            Market::TSE => chrono_tz::Asia::Tokyo,
            Market::TSX => chrono_tz::America::Toronto,
        }
    }

    /// Get the name of this market
    pub fn name(&self) -> &'static str {
        match self {
            Market::NYSE => "New York Stock Exchange",
            Market::NASDAQ => "NASDAQ Stock Market",
            Market::LSE => "London Stock Exchange",
            Market::TSE => "Tokyo Stock Exchange",
            Market::TSX => "Toronto Stock Exchange",
        }
    }

    /// Get the code for this market
    pub fn code(&self) -> &'static str {
        match self {
            Market::NYSE => "NYSE",
            Market::NASDAQ => "NASDAQ",
            Market::LSE => "LSE",
            Market::TSE => "TSE",
            Market::TSX => "TSX",
        }
    }

    /// Create the implementation for this market
    pub(crate) fn create_implementation(&self) -> Result<Box<dyn MarketImpl>> {
        Ok(match self {
            Market::NYSE | Market::NASDAQ => Box::new(us::USMarket::new()),
            Market::LSE => Box::new(uk::LSEMarket::new()),
            Market::TSE => Box::new(japan::TSEMarket::new()),
            Market::TSX => Box::new(canada::TSXMarket::new()),
        })
    }
}

impl fmt::Display for Market {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Check if a date is a weekend
pub(crate) fn is_weekend(date: NaiveDate) -> bool {
    matches!(date.weekday(), Weekday::Sat | Weekday::Sun)
}
