//! US holiday rules and calculations

use crate::utils::{calculate_good_friday, last_weekday_of_month, nth_weekday_of_month};
use chrono::{Datelike, NaiveDate, Weekday};
use std::collections::HashSet;

/// Expected number of US holidays per year
const US_HOLIDAYS_PER_YEAR: usize = 11;

/// Get all US holidays for a given year
pub fn get_us_holidays(year: i32) -> HashSet<NaiveDate> {
    let mut holidays = HashSet::with_capacity(US_HOLIDAYS_PER_YEAR);

    // Fixed holidays with weekend adjustments
    holidays.insert(new_years_day(year));
    holidays.insert(independence_day(year));
    holidays.insert(christmas_day(year));

    // Juneteenth only became a federal holiday in 2021
    if year >= 2021 {
        holidays.insert(juneteenth(year));
    }

    // Variable holidays
    if let Some(date) = mlk_day(year) {
        holidays.insert(date);
    }
    if let Some(date) = presidents_day(year) {
        holidays.insert(date);
    }
    if let Some(date) = memorial_day(year) {
        holidays.insert(date);
    }
    if let Some(date) = labor_day(year) {
        holidays.insert(date);
    }
    if let Some(date) = thanksgiving_day(year) {
        holidays.insert(date);
    }
    if let Ok(date) = calculate_good_friday(year) {
        holidays.insert(date);
    }

    holidays
}

/// New Year's Day (January 1st, observed on Monday if weekend)
pub fn new_years_day(year: i32) -> NaiveDate {
    let jan1 = NaiveDate::from_ymd_opt(year, 1, 1).expect("Valid date");
    match jan1.weekday() {
        Weekday::Sat => jan1 + chrono::Duration::days(2),
        Weekday::Sun => jan1 + chrono::Duration::days(1),
        _ => jan1,
    }
}

/// Martin Luther King Jr. Day (3rd Monday of January)
pub fn mlk_day(year: i32) -> Option<NaiveDate> {
    nth_weekday_of_month(year, 1, Weekday::Mon, 3)
}

/// Presidents' Day (3rd Monday of February)
pub fn presidents_day(year: i32) -> Option<NaiveDate> {
    nth_weekday_of_month(year, 2, Weekday::Mon, 3)
}

/// Memorial Day (Last Monday of May)
pub fn memorial_day(year: i32) -> Option<NaiveDate> {
    last_weekday_of_month(year, 5, Weekday::Mon)
}

/// Independence Day (July 4th, observed on Friday if Saturday, Monday if Sunday)
pub fn independence_day(year: i32) -> NaiveDate {
    let july4 = NaiveDate::from_ymd_opt(year, 7, 4).expect("Valid date");
    match july4.weekday() {
        Weekday::Sat => july4 - chrono::Duration::days(1),
        Weekday::Sun => july4 + chrono::Duration::days(1),
        _ => july4,
    }
}

/// Labor Day (1st Monday of September)
pub fn labor_day(year: i32) -> Option<NaiveDate> {
    nth_weekday_of_month(year, 9, Weekday::Mon, 1)
}

/// Thanksgiving Day (4th Thursday of November)
pub fn thanksgiving_day(year: i32) -> Option<NaiveDate> {
    nth_weekday_of_month(year, 11, Weekday::Thu, 4)
}

/// Juneteenth (June 19th, observed on Monday if weekend)
pub fn juneteenth(year: i32) -> NaiveDate {
    let jun19 = NaiveDate::from_ymd_opt(year, 6, 19).expect("Valid date");
    match jun19.weekday() {
        Weekday::Sat => jun19 + chrono::Duration::days(2),
        Weekday::Sun => jun19 + chrono::Duration::days(1),
        _ => jun19,
    }
}

/// Christmas Day (December 25th, observed on Monday if weekend)
pub fn christmas_day(year: i32) -> NaiveDate {
    let dec25 = NaiveDate::from_ymd_opt(year, 12, 25).expect("Valid date");
    match dec25.weekday() {
        Weekday::Sat => dec25 + chrono::Duration::days(2),
        Weekday::Sun => dec25 + chrono::Duration::days(1),
        _ => dec25,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_years_day() {
        // 2024: January 1st is Monday
        assert_eq!(
            new_years_day(2024),
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()
        );

        // 2023: January 1st is Sunday, so observed on Monday
        assert_eq!(
            new_years_day(2023),
            NaiveDate::from_ymd_opt(2023, 1, 2).unwrap()
        );
    }

    #[test]
    fn test_mlk_day() {
        // 2024: 3rd Monday of January
        assert_eq!(
            mlk_day(2024),
            Some(NaiveDate::from_ymd_opt(2024, 1, 15).unwrap())
        );

        // 2025: 3rd Monday of January
        assert_eq!(
            mlk_day(2025),
            Some(NaiveDate::from_ymd_opt(2025, 1, 20).unwrap())
        );
    }

    #[test]
    fn test_thanksgiving() {
        // 2024: 4th Thursday of November
        assert_eq!(
            thanksgiving_day(2024),
            Some(NaiveDate::from_ymd_opt(2024, 11, 28).unwrap())
        );

        // 2025: 4th Thursday of November
        assert_eq!(
            thanksgiving_day(2025),
            Some(NaiveDate::from_ymd_opt(2025, 11, 27).unwrap())
        );
    }

    #[test]
    fn test_christmas() {
        // 2024: December 25th is Wednesday
        assert_eq!(
            christmas_day(2024),
            NaiveDate::from_ymd_opt(2024, 12, 25).unwrap()
        );

        // 2025: December 25th is Thursday
        assert_eq!(
            christmas_day(2025),
            NaiveDate::from_ymd_opt(2025, 12, 25).unwrap()
        );
    }

    #[test]
    fn test_juneteenth_year_condition() {
        // Juneteenth should not be a holiday before 2021
        let holidays_2020 = get_us_holidays(2020);
        assert!(!holidays_2020.contains(&NaiveDate::from_ymd_opt(2020, 6, 19).unwrap()));

        // Juneteenth should be a holiday from 2021 onwards
        let holidays_2021 = get_us_holidays(2021);
        // June 19, 2021 was Saturday, so observed on Monday June 21
        assert!(holidays_2021.contains(&NaiveDate::from_ymd_opt(2021, 6, 21).unwrap()));

        let holidays_2025 = get_us_holidays(2025);
        assert!(holidays_2025.contains(&NaiveDate::from_ymd_opt(2025, 6, 19).unwrap()));
    }
}
