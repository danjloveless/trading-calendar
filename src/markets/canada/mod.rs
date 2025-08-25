//! Canadian market implementation

pub mod holidays;

use crate::constants::*;
use crate::markets::MarketImpl;
use crate::utils::HolidayCache;
use crate::{Session, TradingHours};
use chrono::{Datelike, NaiveDate};
use chrono_tz::Tz;

/// Canadian market implementation (TSX)
pub struct TSXMarket {
    cache: HolidayCache,
}

impl TSXMarket {
    /// Create a new TSX market instance
    pub fn new() -> Self {
        Self {
            cache: HolidayCache::default(),
        }
    }
}

impl Default for TSXMarket {
    fn default() -> Self {
        Self::new()
    }
}

impl MarketImpl for TSXMarket {
    fn is_holiday(&self, date: NaiveDate) -> bool {
        let year = date.year();
        let holidays = self
            .cache
            .get_or_compute(year, || holidays::get_canada_holidays(year));
        holidays.contains(&date)
    }

    fn trading_hours(&self, date: NaiveDate) -> TradingHours {
        TradingHours::new(
            date,
            Session::new_unchecked(CA_REGULAR_OPEN, CA_REGULAR_CLOSE),
            None, // No pre-market
            None, // No after-hours
        )
    }

    fn timezone(&self) -> Tz {
        chrono_tz::America::Toronto
    }
}
