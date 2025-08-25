//! UK market implementation

pub mod holidays;

use crate::constants::*;
use crate::markets::MarketImpl;
use crate::utils::HolidayCache;
use crate::{Session, TradingHours};
use chrono::{Datelike, NaiveDate};
use chrono_tz::Tz;

/// UK market implementation (LSE)
pub struct LSEMarket {
    cache: HolidayCache,
}

impl LSEMarket {
    /// Create a new LSE market instance
    pub fn new() -> Self {
        Self {
            cache: HolidayCache::default(),
        }
    }
}

impl Default for LSEMarket {
    fn default() -> Self {
        Self::new()
    }
}

impl MarketImpl for LSEMarket {
    fn is_holiday(&self, date: NaiveDate) -> bool {
        let year = date.year();
        let holidays = self
            .cache
            .get_or_compute(year, || holidays::get_uk_holidays(year));
        holidays.contains(&date)
    }

    fn trading_hours(&self, date: NaiveDate) -> TradingHours {
        TradingHours::new(
            date,
            Session::new_unchecked(UK_REGULAR_OPEN, UK_REGULAR_CLOSE),
            None, // No pre-market
            None, // No after-hours
        )
    }

    fn timezone(&self) -> Tz {
        chrono_tz::Europe::London
    }
}
