//! Canadian holiday rules and calculations

use crate::utils::{calculate_good_friday, nth_weekday_of_month};
use chrono::{Datelike, NaiveDate, Weekday};
use std::collections::HashSet;

/// Get all Canadian holidays for a given year
pub fn get_canada_holidays(year: i32) -> HashSet<NaiveDate> {
    let mut holidays = HashSet::with_capacity(9); // Reduced from 12

    // New Year's Day
    holidays.insert(adjust_for_weekend(
        NaiveDate::from_ymd_opt(year, 1, 1).expect("Valid date"),
    ));

    // Family Day - TSX observes this starting from 2008
    if year >= 2008 {
        if let Some(date) = nth_weekday_of_month(year, 2, Weekday::Mon, 3) {
            holidays.insert(date);
        }
    }

    // Good Friday
    if let Ok(date) = calculate_good_friday(year) {
        holidays.insert(date);
    }

    // Victoria Day (Monday on or before May 24)
    if let Some(date) = victoria_day(year) {
        holidays.insert(date);
    }

    // Canada Day (July 1)
    holidays.insert(adjust_for_weekend(
        NaiveDate::from_ymd_opt(year, 7, 1).expect("Valid date"),
    ));

    // NO Civic Holiday - TSX is OPEN

    // Labour Day (1st Monday of September)
    if let Some(date) = nth_weekday_of_month(year, 9, Weekday::Mon, 1) {
        holidays.insert(date);
    }

    // Thanksgiving (2nd Monday of October)
    if let Some(date) = nth_weekday_of_month(year, 10, Weekday::Mon, 2) {
        holidays.insert(date);
    }

    // Christmas Day
    let christmas = NaiveDate::from_ymd_opt(year, 12, 25).expect("Valid date");
    holidays.insert(adjust_for_weekend(christmas));

    // Boxing Day (special rules for Canada)
    let boxing = NaiveDate::from_ymd_opt(year, 12, 26).expect("Valid date");
    match (christmas.weekday(), boxing.weekday()) {
        (Weekday::Fri, Weekday::Sat) => {
            // Christmas on Friday, Boxing Day on Saturday
            // Christmas observed on Friday, Boxing Day observed on Monday
            holidays.insert(boxing + chrono::Duration::days(2));
        }
        (Weekday::Sat, Weekday::Sun) => {
            // Christmas on Saturday, Boxing Day on Sunday
            // Christmas observed on Monday, Boxing Day observed on Tuesday
            holidays.insert(boxing + chrono::Duration::days(2));
        }
        (_, Weekday::Sat) => {
            holidays.insert(boxing + chrono::Duration::days(2));
        }
        (_, Weekday::Sun) => {
            holidays.insert(boxing + chrono::Duration::days(1));
        }
        _ => {
            holidays.insert(boxing);
        }
    };

    holidays
}

fn victoria_day(year: i32) -> Option<NaiveDate> {
    let may_24 = NaiveDate::from_ymd_opt(year, 5, 24)?;
    let mut victoria = may_24;
    while victoria.weekday() != Weekday::Mon {
        victoria -= chrono::Duration::days(1);
    }
    Some(victoria)
}

fn adjust_for_weekend(date: NaiveDate) -> NaiveDate {
    match date.weekday() {
        Weekday::Sat => date + chrono::Duration::days(2),
        Weekday::Sun => date + chrono::Duration::days(1),
        _ => date,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canada_holidays_2025() {
        let holidays = get_canada_holidays(2025);

        // New Year's Day 2025 (Wednesday) - should be observed on Wednesday
        assert!(holidays.contains(&NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()));

        // Family Day 2025 (Feb 17)
        assert!(holidays.contains(&NaiveDate::from_ymd_opt(2025, 2, 17).unwrap()));

        // Good Friday 2025 (April 18)
        assert!(holidays.contains(&NaiveDate::from_ymd_opt(2025, 4, 18).unwrap()));

        // Victoria Day 2025 (May 19)
        assert!(holidays.contains(&NaiveDate::from_ymd_opt(2025, 5, 19).unwrap()));

        // Canada Day 2025 (July 1)
        assert!(holidays.contains(&NaiveDate::from_ymd_opt(2025, 7, 1).unwrap()));

        // Civic Holiday 2025 (Aug 4) - NOT a TSX holiday
        assert!(!holidays.contains(&NaiveDate::from_ymd_opt(2025, 8, 4).unwrap()));

        // Labour Day 2025 (Sept 1)
        assert!(holidays.contains(&NaiveDate::from_ymd_opt(2025, 9, 1).unwrap()));

        // Thanksgiving 2025 (Oct 13)
        assert!(holidays.contains(&NaiveDate::from_ymd_opt(2025, 10, 13).unwrap()));

        // Christmas Day 2025 (Thursday) - should be observed on Thursday
        assert!(holidays.contains(&NaiveDate::from_ymd_opt(2025, 12, 25).unwrap()));

        // Boxing Day 2025 (Friday) - should be observed on Friday
        assert!(holidays.contains(&NaiveDate::from_ymd_opt(2025, 12, 26).unwrap()));
    }

    #[test]
    fn test_victoria_day_calculation() {
        // Victoria Day is the Monday on or before May 24

        // 2025: May 24 is a Saturday, so Victoria Day is May 19 (Monday)
        let victoria_2025 = victoria_day(2025).unwrap();
        assert_eq!(victoria_2025, NaiveDate::from_ymd_opt(2025, 5, 19).unwrap());
        assert_eq!(victoria_2025.weekday(), Weekday::Mon);

        // 2026: May 24 is a Sunday, so Victoria Day is May 18 (Monday)
        let victoria_2026 = victoria_day(2026).unwrap();
        assert_eq!(victoria_2026, NaiveDate::from_ymd_opt(2026, 5, 18).unwrap());
        assert_eq!(victoria_2026.weekday(), Weekday::Mon);

        // 2027: May 24 is a Monday, so Victoria Day is May 24 (Monday)
        let victoria_2027 = victoria_day(2027).unwrap();
        assert_eq!(victoria_2027, NaiveDate::from_ymd_opt(2027, 5, 24).unwrap());
        assert_eq!(victoria_2027.weekday(), Weekday::Mon);
    }

    #[test]
    fn test_canada_weekend_adjustments() {
        // Test Canada Day falling on Sunday (should move to Monday)
        let holidays_2023 = get_canada_holidays(2023);
        // Canada Day 2023 was Saturday, so should be observed on Monday Jul 3
        assert!(holidays_2023.contains(&NaiveDate::from_ymd_opt(2023, 7, 3).unwrap()));

        // Test Christmas Day falling on Sunday (should move to Monday)
        let holidays_2022 = get_canada_holidays(2022);
        // Christmas Day 2022 was Sunday, so should be observed on Monday Dec 26
        assert!(holidays_2022.contains(&NaiveDate::from_ymd_opt(2022, 12, 26).unwrap()));
    }
}
