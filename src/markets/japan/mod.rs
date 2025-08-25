//! Japanese market implementation

pub mod holidays;

use crate::constants::*;
use crate::markets::MarketImpl;
use crate::utils::HolidayCache;
use crate::{Session, TradingHours};
use chrono::{Datelike, NaiveDate};
use chrono_tz::Tz;

/// Japanese market implementation (TSE)
pub struct TSEMarket {
    cache: HolidayCache,
}

impl TSEMarket {
    /// Create a new TSE market instance
    pub fn new() -> Self {
        Self {
            cache: HolidayCache::default(),
        }
    }
}

impl Default for TSEMarket {
    fn default() -> Self {
        Self::new()
    }
}

impl MarketImpl for TSEMarket {
    fn is_holiday(&self, date: NaiveDate) -> bool {
        let year = date.year();
        let holidays = self
            .cache
            .get_or_compute(year, || holidays::get_japan_holidays(year));
        holidays.contains(&date)
    }

    fn trading_hours(&self, date: NaiveDate) -> TradingHours {
        TradingHours::new(
            date,
            Session::new_unchecked(JP_REGULAR_OPEN, JP_REGULAR_CLOSE),
            None, // No pre-market
            None, // No after-hours
        )
    }

    fn timezone(&self) -> Tz {
        chrono_tz::Asia::Tokyo
    }
}
