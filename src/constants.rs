//! Market-specific constants
//!
//! # Safety
//!
//! All time constants in this module are hardcoded and known to be valid.
//! The unwrap() calls are safe because these are compile-time constants
//! with values that are guaranteed to be valid NaiveTime instances.

use chrono::NaiveTime;

// US Market Times
pub const US_REGULAR_OPEN: NaiveTime = NaiveTime::from_hms_opt(9, 30, 0).unwrap();

pub const US_REGULAR_CLOSE: NaiveTime = NaiveTime::from_hms_opt(16, 0, 0).unwrap();

pub const US_PREMARKET_OPEN: NaiveTime = NaiveTime::from_hms_opt(4, 0, 0).unwrap();

pub const US_AFTERHOURS_CLOSE: NaiveTime = NaiveTime::from_hms_opt(20, 0, 0).unwrap();

pub const US_EARLY_CLOSE: NaiveTime = NaiveTime::from_hms_opt(13, 0, 0).unwrap();

// UK Market Times
pub const UK_REGULAR_OPEN: NaiveTime = NaiveTime::from_hms_opt(8, 0, 0).unwrap();

pub const UK_REGULAR_CLOSE: NaiveTime = NaiveTime::from_hms_opt(16, 30, 0).unwrap();

pub const UK_EARLY_CLOSE: NaiveTime = NaiveTime::from_hms_opt(12, 30, 0).unwrap();

// Japan Market Times
pub const JP_REGULAR_OPEN: NaiveTime = NaiveTime::from_hms_opt(9, 0, 0).unwrap();

pub const JP_REGULAR_CLOSE: NaiveTime = NaiveTime::from_hms_opt(15, 0, 0).unwrap();

// Canada Market Times
pub const CA_REGULAR_OPEN: NaiveTime = NaiveTime::from_hms_opt(9, 30, 0).unwrap();

pub const CA_REGULAR_CLOSE: NaiveTime = NaiveTime::from_hms_opt(16, 0, 0).unwrap();

pub const CA_EARLY_CLOSE: NaiveTime = NaiveTime::from_hms_opt(13, 0, 0).unwrap();
