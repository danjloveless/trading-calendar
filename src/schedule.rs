//! Trading hours and session definitions

use crate::{CalendarError, Result};
use chrono::{NaiveDate, NaiveTime};
use std::fmt;

/// A trading session with start and end times
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Session {
    /// Start time of the session
    pub start: NaiveTime,
    /// End time of the session
    pub end: NaiveTime,
}

impl Session {
    /// Create a new trading session
    /// For overnight sessions (e.g., after-hours ending at midnight), end can be "before" start
    pub fn new(start: NaiveTime, end: NaiveTime) -> Result<Self> {
        // Allow end == start for 24-hour sessions
        // Allow end < start for overnight sessions
        if end == start {
            return Err(CalendarError::InvalidSession);
        }
        Ok(Session { start, end })
    }

    /// Create a new session without validation (internal use)
    pub(crate) fn new_unchecked(start: NaiveTime, end: NaiveTime) -> Self {
        Session { start, end }
    }

    /// Check if a time falls within this session (handles overnight sessions)
    pub fn contains(&self, time: NaiveTime) -> bool {
        if self.start < self.end {
            // Normal session (e.g., 9:30 AM - 4:00 PM)
            time >= self.start && time < self.end
        } else {
            // Overnight session (e.g., 8:00 PM - 4:00 AM)
            time >= self.start || time < self.end
        }
    }
}

impl fmt::Display for Session {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} - {}",
            self.start.format("%H:%M"),
            self.end.format("%H:%M")
        )
    }
}

/// Trading hours for a specific date
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TradingHours {
    /// The date these hours apply to
    pub date: NaiveDate,
    /// Pre-market session (if available)
    pub pre_market: Option<Session>,
    /// Regular trading session
    pub regular: Session,
    /// After-hours session (if available)
    pub after_hours: Option<Session>,
    /// Early close time (overrides regular session end)
    pub early_close: Option<NaiveTime>,
}

impl TradingHours {
    /// Create a new TradingHours instance
    pub fn new(
        date: NaiveDate,
        regular: Session,
        pre_market: Option<Session>,
        after_hours: Option<Session>,
    ) -> Self {
        TradingHours {
            date,
            pre_market,
            regular,
            after_hours,
            early_close: None,
        }
    }

    /// Set early close time with validation
    pub fn with_early_close(mut self, early_close: NaiveTime) -> Result<Self> {
        if early_close >= self.regular.end {
            return Err(CalendarError::InvalidTime(
                "Early close must be before regular close".to_string(),
            ));
        }
        self.early_close = Some(early_close);
        Ok(self)
    }

    /// Check if this is an early close day
    pub fn is_early_close(&self) -> bool {
        self.early_close.is_some()
    }

    /// Get the market close time (considering early close)
    pub fn market_close(&self) -> NaiveTime {
        self.early_close.unwrap_or(self.regular.end)
    }

    /// Check if the market is open at a specific time
    pub fn is_open_at(&self, time: NaiveTime) -> bool {
        // Check pre-market session
        if let Some(ref pre) = self.pre_market {
            if pre.contains(time) {
                return true;
            }
        }

        // Check regular session (with early close consideration)
        let regular_end = self.early_close.unwrap_or(self.regular.end);
        if time >= self.regular.start && time < regular_end {
            return true;
        }

        // Check after-hours session
        // After-hours is available on early close days, but only after the early close time
        if let Some(ref after) = self.after_hours {
            if after.contains(time) {
                // If it's an early close day, only allow after-hours after early close
                if let Some(early_close) = self.early_close {
                    if time >= early_close {
                        return true;
                    }
                } else {
                    // No early close, after-hours is available
                    return true;
                }
            }
        }

        false
    }
}

impl fmt::Display for TradingHours {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: Regular {}", self.date, self.regular)?;

        if let Some(ref pre) = self.pre_market {
            write!(f, ", Pre-Market {pre}")?;
        }

        if let Some(ref after) = self.after_hours {
            write!(f, ", After-Hours {after}")?;
        }

        if let Some(early) = self.early_close {
            write!(f, " (Early Close: {})", early.format("%H:%M"))?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_contains() {
        let session = Session::new_unchecked(
            NaiveTime::from_hms_opt(9, 30, 0).unwrap(),
            NaiveTime::from_hms_opt(16, 0, 0).unwrap(),
        );

        assert!(session.contains(NaiveTime::from_hms_opt(10, 0, 0).unwrap()));
        assert!(session.contains(NaiveTime::from_hms_opt(9, 30, 0).unwrap()));
        assert!(!session.contains(NaiveTime::from_hms_opt(16, 0, 0).unwrap()));
        assert!(!session.contains(NaiveTime::from_hms_opt(9, 29, 59).unwrap()));
    }

    #[test]
    fn test_early_close() {
        let hours = TradingHours {
            date: NaiveDate::from_ymd_opt(2025, 12, 24).unwrap(),
            pre_market: None,
            regular: Session::new_unchecked(
                NaiveTime::from_hms_opt(9, 30, 0).unwrap(),
                NaiveTime::from_hms_opt(16, 0, 0).unwrap(),
            ),
            after_hours: Some(Session::new_unchecked(
                NaiveTime::from_hms_opt(16, 0, 0).unwrap(),
                NaiveTime::from_hms_opt(20, 0, 0).unwrap(),
            )),
            early_close: Some(NaiveTime::from_hms_opt(13, 0, 0).unwrap()),
        };

        assert!(hours.is_early_close());
        assert_eq!(
            hours.market_close(),
            NaiveTime::from_hms_opt(13, 0, 0).unwrap()
        );

        // Should be open before early close
        assert!(hours.is_open_at(NaiveTime::from_hms_opt(12, 0, 0).unwrap()));

        // Should be closed between early close and after-hours start
        assert!(!hours.is_open_at(NaiveTime::from_hms_opt(14, 0, 0).unwrap()));

        // Should be open during after-hours (after early close)
        assert!(hours.is_open_at(NaiveTime::from_hms_opt(17, 0, 0).unwrap()));
    }

    #[test]
    fn test_overnight_session() {
        // Test overnight session (e.g., 8:00 PM - 4:00 AM)
        let session = Session::new(
            NaiveTime::from_hms_opt(20, 0, 0).unwrap(),
            NaiveTime::from_hms_opt(4, 0, 0).unwrap(),
        )
        .expect("Overnight session should be valid");

        // Should contain times after start
        assert!(session.contains(NaiveTime::from_hms_opt(22, 0, 0).unwrap()));
        assert!(session.contains(NaiveTime::from_hms_opt(2, 0, 0).unwrap()));

        // Should not contain times in the middle of the day
        assert!(!session.contains(NaiveTime::from_hms_opt(12, 0, 0).unwrap()));
        assert!(!session.contains(NaiveTime::from_hms_opt(16, 0, 0).unwrap()));
    }

    #[test]
    fn test_session_validation() {
        // Test invalid session (start == end)
        let result = Session::new(
            NaiveTime::from_hms_opt(9, 30, 0).unwrap(),
            NaiveTime::from_hms_opt(9, 30, 0).unwrap(),
        );
        assert!(result.is_err());

        // Test valid normal session
        let result = Session::new(
            NaiveTime::from_hms_opt(9, 30, 0).unwrap(),
            NaiveTime::from_hms_opt(16, 0, 0).unwrap(),
        );
        assert!(result.is_ok());

        // Test valid overnight session
        let result = Session::new(
            NaiveTime::from_hms_opt(20, 0, 0).unwrap(),
            NaiveTime::from_hms_opt(4, 0, 0).unwrap(),
        );
        assert!(result.is_ok());
    }
}
