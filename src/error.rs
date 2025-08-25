//! Error types for the trading calendar

use chrono::NaiveDate;
use thiserror::Error;

/// Errors that can occur when using the trading calendar
#[derive(Error, Debug)]
pub enum CalendarError {
    /// Date is outside the supported range
    #[error("Date {0} is outside supported range (2020-2030). Please use a date within the supported range.")]
    DateOutOfRange(NaiveDate),

    /// Invalid time provided
    #[error("Invalid time for market operation: {0}. Times must be in 24-hour format (HH:MM:SS).")]
    InvalidTime(String),

    /// No trading day found within search period
    #[error("No trading day found within search period. The market may be closed for an extended period.")]
    NoTradingDayFound,

    /// Invalid date calculation
    #[error("Invalid date calculation: {0}")]
    InvalidDateCalculation(String),

    /// Invalid configuration
    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),

    /// Invalid session times
    #[error("Invalid session: end time must be after start time for regular sessions")]
    InvalidSession,
}

/// Result type alias for trading calendar operations
pub type Result<T> = std::result::Result<T, CalendarError>;
