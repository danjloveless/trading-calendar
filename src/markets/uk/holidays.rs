//! UK holiday rules and calculations

use crate::utils::{
    calculate_easter_monday, calculate_good_friday, last_weekday_of_month, nth_weekday_of_month,
};
use chrono::{Datelike, NaiveDate, Weekday};
use std::collections::HashSet;

/// Expected number of UK holidays per year
const UK_HOLIDAYS_PER_YEAR: usize = 10;

/// Get all UK holidays for a given year
pub fn get_uk_holidays(year: i32) -> HashSet<NaiveDate> {
    let mut holidays = HashSet::with_capacity(UK_HOLIDAYS_PER_YEAR);

    // New Year's Day
    holidays.insert(adjust_for_weekend_uk(
        NaiveDate::from_ymd_opt(year, 1, 1).expect("Valid date"),
    ));

    // Good Friday
    if let Ok(date) = calculate_good_friday(year) {
        holidays.insert(date);
    }

    // Easter Monday
    if let Ok(date) = calculate_easter_monday(year) {
        holidays.insert(date);
    }

    // Early May Bank Holiday (1st Monday of May)
    if let Some(date) = nth_weekday_of_month(year, 5, Weekday::Mon, 1) {
        holidays.insert(date);
    }

    // Spring Bank Holiday (last Monday of May)
    if let Some(date) = last_weekday_of_month(year, 5, Weekday::Mon) {
        holidays.insert(date);
    }

    // Summer Bank Holiday (last Monday of August)
    if let Some(date) = last_weekday_of_month(year, 8, Weekday::Mon) {
        holidays.insert(date);
    }

    // Christmas Day
    let christmas = NaiveDate::from_ymd_opt(year, 12, 25).expect("Valid date");
    holidays.insert(adjust_for_weekend_uk(christmas));

    // Boxing Day (special rules)
    add_boxing_day(&mut holidays, year);

    holidays
}

fn adjust_for_weekend_uk(date: NaiveDate) -> NaiveDate {
    match date.weekday() {
        Weekday::Sat => date + chrono::Duration::days(2),
        Weekday::Sun => date + chrono::Duration::days(1),
        _ => date,
    }
}

fn add_boxing_day(holidays: &mut HashSet<NaiveDate>, year: i32) {
    let christmas = NaiveDate::from_ymd_opt(year, 12, 25).expect("Valid date");
    let boxing = NaiveDate::from_ymd_opt(year, 12, 26).expect("Valid date");

    match christmas.weekday() {
        Weekday::Fri => {
            // Christmas on Friday = observed Friday
            // Boxing Day on Saturday = observed Monday
            holidays.insert(christmas);
            holidays.insert(boxing + chrono::Duration::days(2));
        }
        Weekday::Sat => {
            // Christmas on Saturday = observed Monday (27th)
            // Boxing Day on Sunday = observed Tuesday (28th)
            holidays.insert(NaiveDate::from_ymd_opt(year, 12, 27).unwrap());
            holidays.insert(NaiveDate::from_ymd_opt(year, 12, 28).unwrap());
        }
        Weekday::Sun => {
            // Christmas on Sunday = observed Monday (26th)
            // Boxing Day on Monday = observed Tuesday (27th)
            holidays.insert(boxing);
            holidays.insert(NaiveDate::from_ymd_opt(year, 12, 27).unwrap());
        }
        _ => {
            // Christmas on weekday
            holidays.insert(christmas);
            holidays.insert(adjust_for_weekend_uk(boxing));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uk_holidays_2025() {
        let holidays = get_uk_holidays(2025);

        // New Year's Day 2025 (Wednesday) - no adjustment needed
        assert!(holidays.contains(&NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()));

        // Good Friday 2025 (April 18)
        assert!(holidays.contains(&NaiveDate::from_ymd_opt(2025, 4, 18).unwrap()));

        // Easter Monday 2025 (April 21)
        assert!(holidays.contains(&NaiveDate::from_ymd_opt(2025, 4, 21).unwrap()));

        // Early May Bank Holiday 2025 (May 5)
        assert!(holidays.contains(&NaiveDate::from_ymd_opt(2025, 5, 5).unwrap()));

        // Spring Bank Holiday 2025 (May 26)
        assert!(holidays.contains(&NaiveDate::from_ymd_opt(2025, 5, 26).unwrap()));

        // Summer Bank Holiday 2025 (August 25)
        assert!(holidays.contains(&NaiveDate::from_ymd_opt(2025, 8, 25).unwrap()));

        // Christmas Day 2025 (Thursday) - no adjustment needed
        assert!(holidays.contains(&NaiveDate::from_ymd_opt(2025, 12, 25).unwrap()));

        // Boxing Day 2025 (Friday) - no adjustment needed
        assert!(holidays.contains(&NaiveDate::from_ymd_opt(2025, 12, 26).unwrap()));
    }

    #[test]
    fn test_weekend_adjustments() {
        // Test New Year's Day falling on Saturday (should move to Monday)
        let saturday_ny = NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(); // Saturday
        let adjusted = adjust_for_weekend_uk(saturday_ny);
        assert_eq!(adjusted, NaiveDate::from_ymd_opt(2022, 1, 3).unwrap()); // Monday

        // Test Boxing Day falling on Sunday (should move to Monday for general rule)
        let sunday_boxing = NaiveDate::from_ymd_opt(2021, 12, 26).unwrap(); // Sunday
        let adjusted = adjust_for_weekend_uk(sunday_boxing);
        assert_eq!(adjusted, NaiveDate::from_ymd_opt(2021, 12, 27).unwrap()); // Monday
    }

    #[test]
    fn test_boxing_day_logic() {
        // Test 2021: Christmas on Saturday, Boxing Day on Sunday
        let holidays_2021 = get_uk_holidays(2021);
        assert!(holidays_2021.contains(&NaiveDate::from_ymd_opt(2021, 12, 27).unwrap())); // Christmas observed Monday
        assert!(holidays_2021.contains(&NaiveDate::from_ymd_opt(2021, 12, 28).unwrap())); // Boxing Day observed Tuesday

        // Test 2022: Christmas on Sunday, Boxing Day on Monday
        let holidays_2022 = get_uk_holidays(2022);
        assert!(holidays_2022.contains(&NaiveDate::from_ymd_opt(2022, 12, 26).unwrap())); // Christmas observed Monday
        assert!(holidays_2022.contains(&NaiveDate::from_ymd_opt(2022, 12, 27).unwrap())); // Boxing Day observed Tuesday

        // Test 2025: Christmas on Thursday, Boxing Day on Friday (no adjustment needed)
        let holidays_2025 = get_uk_holidays(2025);
        assert!(holidays_2025.contains(&NaiveDate::from_ymd_opt(2025, 12, 25).unwrap())); // Christmas
        assert!(holidays_2025.contains(&NaiveDate::from_ymd_opt(2025, 12, 26).unwrap()));
        // Boxing Day
    }
}
