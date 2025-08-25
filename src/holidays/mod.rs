//! Holiday management module

use chrono::{NaiveDate, NaiveTime};

/// Represents a holiday with additional information
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Holiday {
    /// The date of the holiday
    pub date: NaiveDate,
    /// The name of the holiday
    pub name: &'static str,
    /// Whether the market is closed on this holiday
    pub market_closed: bool,
    /// Early close time if the market closes early on this holiday
    pub early_close: Option<NaiveTime>,
}

impl Holiday {
    /// Create a new holiday
    pub fn new(date: NaiveDate, name: &'static str, market_closed: bool) -> Self {
        Self {
            date,
            name,
            market_closed,
            early_close: None,
        }
    }

    /// Create a new holiday with early close time
    pub fn with_early_close(date: NaiveDate, name: &'static str, early_close: NaiveTime) -> Self {
        Self {
            date,
            name,
            market_closed: false,
            early_close: Some(early_close),
        }
    }
}





