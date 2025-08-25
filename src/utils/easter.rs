//! Easter calculation algorithms

use crate::{CalendarError, Result};
use chrono::NaiveDate;

/// Calculate Easter Sunday using the Anonymous Gregorian algorithm
pub fn calculate_easter(year: i32) -> Result<NaiveDate> {
    let a = year % 19;
    let b = year / 100;
    let c = year % 100;
    let d = b / 4;
    let e = b % 4;
    let f = (b + 8) / 25;
    let g = (b - f + 1) / 3;
    let h = (19 * a + b - d - g + 15) % 30;
    let i = c / 4;
    let k = c % 4;
    let l = (32 + 2 * e + 2 * i - h - k) % 7;
    let m = (a + 11 * h + 22 * l) / 451;
    let month = (h + l - 7 * m + 114) / 31;
    let day = ((h + l - 7 * m + 114) % 31) + 1;

    NaiveDate::from_ymd_opt(year, month as u32, day as u32).ok_or_else(|| {
        CalendarError::InvalidDateCalculation(format!("Easter calculation failed for year {year}"))
    })
}

/// Calculate Good Friday (2 days before Easter)
pub fn calculate_good_friday(year: i32) -> Result<NaiveDate> {
    Ok(calculate_easter(year)? - chrono::Duration::days(2))
}

/// Calculate Easter Monday (1 day after Easter)
pub fn calculate_easter_monday(year: i32) -> Result<NaiveDate> {
    Ok(calculate_easter(year)? + chrono::Duration::days(1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_easter_dates() {
        // Known Easter dates
        assert_eq!(
            calculate_easter(2024).unwrap(),
            NaiveDate::from_ymd_opt(2024, 3, 31).unwrap()
        );
        assert_eq!(
            calculate_easter(2025).unwrap(),
            NaiveDate::from_ymd_opt(2025, 4, 20).unwrap()
        );
        assert_eq!(
            calculate_easter(2026).unwrap(),
            NaiveDate::from_ymd_opt(2026, 4, 5).unwrap()
        );
        assert_eq!(
            calculate_easter(2027).unwrap(),
            NaiveDate::from_ymd_opt(2027, 3, 28).unwrap()
        );
    }

    #[test]
    fn test_good_friday() {
        assert_eq!(
            calculate_good_friday(2025).unwrap(),
            NaiveDate::from_ymd_opt(2025, 4, 18).unwrap()
        );
        assert_eq!(
            calculate_good_friday(2026).unwrap(),
            NaiveDate::from_ymd_opt(2026, 4, 3).unwrap()
        );
    }
}
