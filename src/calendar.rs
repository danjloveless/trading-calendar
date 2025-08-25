//! Main trading calendar implementation

use crate::markets::MarketImpl;
use crate::{CalendarError, Market, Result, TradingHours, MAX_YEAR, MIN_YEAR};
use chrono::{DateTime, Datelike, NaiveDate, TimeZone, Utc};
use chrono_tz::Tz;

/// A trading calendar for a specific market
pub struct TradingCalendar {
    market: Market,
    implementation: Box<dyn MarketImpl>,
}

impl TradingCalendar {
    /// Create a new trading calendar for the specified market
    pub fn new(market: Market) -> Result<Self> {
        let implementation = market.create_implementation()?;
        Ok(TradingCalendar {
            market,
            implementation,
        })
    }

    /// Check if a specific date is a trading day
    pub fn is_trading_day(&self, date: NaiveDate) -> Result<bool> {
        if date.year() < MIN_YEAR || date.year() > MAX_YEAR {
            return Err(CalendarError::DateOutOfRange(date));
        }
        Ok(self.implementation.is_trading_day(date))
    }

    /// Check if a specific date is a holiday
    pub fn is_holiday(&self, date: NaiveDate) -> Result<bool> {
        if date.year() < MIN_YEAR || date.year() > MAX_YEAR {
            return Err(CalendarError::DateOutOfRange(date));
        }
        Ok(self.implementation.is_holiday(date))
    }

    /// Get trading hours for a specific date
    pub fn trading_hours(&self, date: NaiveDate) -> TradingHours {
        self.implementation.trading_hours(date)
    }

    /// Get the next trading day from a given date
    pub fn next_trading_day(&self, date: NaiveDate) -> NaiveDate {
        self.implementation.next_trading_day(date)
    }

    /// Get the previous trading day from a given date
    pub fn previous_trading_day(&self, date: NaiveDate) -> NaiveDate {
        self.implementation.previous_trading_day(date)
    }

    /// Get the market this calendar is for
    pub fn market(&self) -> Market {
        self.market
    }

    /// Get the timezone for this market
    pub fn timezone(&self) -> Tz {
        self.implementation.timezone()
    }

    /// Check if the market is currently open
    pub fn is_open_now(&self) -> Result<bool> {
        let now = Utc::now().with_timezone(&self.timezone());
        let date = now.date_naive();

        if !self.is_trading_day(date)? {
            return Ok(false);
        }

        let hours = self.trading_hours(date);
        Ok(hours.is_open_at(now.time()))
    }

    /// Get the next time the market opens
    pub fn next_open(&self) -> Result<DateTime<Tz>> {
        let now = Utc::now().with_timezone(&self.timezone());
        let mut date = now.date_naive();

        if self.is_trading_day(date)? {
            let hours = self.trading_hours(date);
            if now.time() < hours.regular.start {
                let dt = date.and_time(hours.regular.start);
                return self
                    .timezone()
                    .from_local_datetime(&dt)
                    .earliest()
                    .ok_or_else(|| {
                        CalendarError::InvalidTime(
                            "Invalid timezone conversion for market open".to_string(),
                        )
                    });
            }
        }

        date = self.next_trading_day(date);
        let hours = self.trading_hours(date);
        let dt = date.and_time(hours.regular.start);

        self.timezone()
            .from_local_datetime(&dt)
            .earliest()
            .ok_or_else(|| {
                CalendarError::InvalidTime(
                    "Invalid timezone conversion for market open".to_string(),
                )
            })
    }

    /// Get the next time the market closes
    pub fn next_close(&self) -> Result<DateTime<Tz>> {
        let now = Utc::now().with_timezone(&self.timezone());
        let mut date = now.date_naive();

        // Check if market is open today
        if self.is_trading_day(date)? {
            let hours = self.trading_hours(date);
            let close_time = hours.market_close();

            if now.time() < close_time {
                let dt = date.and_time(close_time);
                return self
                    .timezone()
                    .from_local_datetime(&dt)
                    .earliest()
                    .ok_or_else(|| {
                        CalendarError::InvalidTime(
                            "Invalid timezone conversion for market close".to_string(),
                        )
                    });
            }
        }

        // Market is closed today, find next trading day
        date = self.next_trading_day(date);
        let hours = self.trading_hours(date);
        let dt = date.and_time(hours.market_close());

        self.timezone()
            .from_local_datetime(&dt)
            .earliest()
            .ok_or_else(|| {
                CalendarError::InvalidTime(
                    "Invalid timezone conversion for market close".to_string(),
                )
            })
    }
}

impl Default for TradingCalendar {
    fn default() -> Self {
        Self::new(Market::NYSE).expect("NYSE calendar should always be valid")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trading_calendar_creation() {
        let calendar = TradingCalendar::new(Market::NYSE).unwrap();
        assert_eq!(calendar.market(), Market::NYSE);
    }

    #[test]
    fn test_trading_day_check() {
        let calendar = TradingCalendar::new(Market::NYSE).unwrap();

        // New Year's Day 2025 - should be a holiday
        assert!(!calendar
            .is_trading_day(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap())
            .unwrap());

        // Regular trading day
        assert!(calendar
            .is_trading_day(NaiveDate::from_ymd_opt(2025, 1, 2).unwrap())
            .unwrap());

        // Weekend
        assert!(!calendar
            .is_trading_day(NaiveDate::from_ymd_opt(2025, 1, 4).unwrap())
            .unwrap()); // Saturday
        assert!(!calendar
            .is_trading_day(NaiveDate::from_ymd_opt(2025, 1, 5).unwrap())
            .unwrap()); // Sunday
    }

    #[test]
    fn test_next_trading_day() {
        let calendar = TradingCalendar::new(Market::NYSE).unwrap();

        // Friday to Monday
        let friday = NaiveDate::from_ymd_opt(2025, 1, 3).unwrap();
        let next = calendar.next_trading_day(friday);
        assert_eq!(next, NaiveDate::from_ymd_opt(2025, 1, 6).unwrap()); // Monday

        // Monday to Tuesday
        let monday = NaiveDate::from_ymd_opt(2025, 1, 6).unwrap();
        let next = calendar.next_trading_day(monday);
        assert_eq!(next, NaiveDate::from_ymd_opt(2025, 1, 7).unwrap()); // Tuesday
    }

    #[test]
    fn test_year_range_validation() {
        let calendar = TradingCalendar::new(Market::NYSE).unwrap();

        // Test dates outside supported range
        let early_date = NaiveDate::from_ymd_opt(2019, 1, 1).unwrap();
        let late_date = NaiveDate::from_ymd_opt(2031, 1, 1).unwrap();

        assert!(calendar.is_trading_day(early_date).is_err());
        assert!(calendar.is_holiday(early_date).is_err());
        assert!(calendar.is_trading_day(late_date).is_err());
        assert!(calendar.is_holiday(late_date).is_err());

        // Test dates within supported range
        let valid_date = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        assert!(calendar.is_trading_day(valid_date).is_ok());
        assert!(calendar.is_holiday(valid_date).is_ok());
    }
}
