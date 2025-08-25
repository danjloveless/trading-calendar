//! US market implementation

pub mod holidays;

use crate::constants::*;
use crate::markets::MarketImpl;
use crate::utils::HolidayCache;
use crate::{Session, TradingHours};
use chrono::{Datelike, NaiveDate};
use chrono_tz::Tz;

/// US market implementation (NYSE/NASDAQ)
pub struct USMarket {
    cache: HolidayCache,
}

impl USMarket {
    /// Create a new US market instance
    pub fn new() -> Self {
        Self {
            cache: HolidayCache::default(),
        }
    }

    /// Check if date is an early close day
    fn is_early_close_day(&self, date: NaiveDate) -> bool {
        // Day after Thanksgiving (Black Friday)
        if let Some(thanksgiving) = holidays::thanksgiving_day(date.year()) {
            let black_friday = thanksgiving + chrono::Duration::days(1);
            if date == black_friday {
                return true;
            }
        }

        // July 3rd if July 4th is a weekday
        if date.month() == 7 && date.day() == 3 {
            let july_4th = date + chrono::Duration::days(1);
            if !crate::markets::is_weekend(july_4th) {
                return true;
            }
        }

        // Christmas Eve if it's a weekday
        if date.month() == 12 && date.day() == 24 && !crate::markets::is_weekend(date) {
            return true;
        }

        false
    }
}

impl Default for USMarket {
    fn default() -> Self {
        Self::new()
    }
}

impl MarketImpl for USMarket {
    fn is_holiday(&self, date: NaiveDate) -> bool {
        let year = date.year();
        let holidays = self
            .cache
            .get_or_compute(year, || holidays::get_us_holidays(year));
        holidays.contains(&date)
    }

    fn trading_hours(&self, date: NaiveDate) -> TradingHours {
        let mut hours = TradingHours::new(
            date,
            Session::new_unchecked(US_REGULAR_OPEN, US_REGULAR_CLOSE),
            Some(Session::new_unchecked(US_PREMARKET_OPEN, US_REGULAR_OPEN)),
            Some(Session::new_unchecked(
                US_REGULAR_CLOSE,
                US_AFTERHOURS_CLOSE,
            )),
        );

        if self.is_early_close_day(date) {
            hours.early_close = Some(US_EARLY_CLOSE);
            // Update after-hours to start at early close time
            hours.after_hours = Some(Session::new_unchecked(US_EARLY_CLOSE, US_AFTERHOURS_CLOSE));
        }

        hours
    }

    fn timezone(&self) -> Tz {
        chrono_tz::America::New_York
    }
}
