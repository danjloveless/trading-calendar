//! Japanese holiday rules and calculations

use crate::utils::nth_weekday_of_month;
use chrono::{Datelike, NaiveDate, Weekday};
use std::collections::HashSet;

/// Expected number of Japanese holidays per year
const JAPAN_HOLIDAYS_PER_YEAR: usize = 20;

/// Get all Japanese holidays for a given year
pub fn get_japan_holidays(year: i32) -> HashSet<NaiveDate> {
    let mut holidays = HashSet::with_capacity(JAPAN_HOLIDAYS_PER_YEAR);

    // New Year holidays (Jan 1-3)
    for day in 1..=3 {
        let date = NaiveDate::from_ymd_opt(year, 1, day).expect("Valid date");
        add_with_substitute(&mut holidays, date);
    }

    // Coming of Age Day (2nd Monday of January)
    if let Some(date) = nth_weekday_of_month(year, 1, Weekday::Mon, 2) {
        holidays.insert(date);
    }

    // National Foundation Day (Feb 11)
    add_with_substitute(
        &mut holidays,
        NaiveDate::from_ymd_opt(year, 2, 11).expect("Valid date"),
    );

    // Emperor's Birthday (Feb 23)
    add_with_substitute(
        &mut holidays,
        NaiveDate::from_ymd_opt(year, 2, 23).expect("Valid date"),
    );

    // Vernal Equinox (around March 20-21)
    if let Some(date) = calculate_vernal_equinox(year) {
        add_with_substitute(&mut holidays, date);
    }

    // Showa Day (Apr 29)
    add_with_substitute(
        &mut holidays,
        NaiveDate::from_ymd_opt(year, 4, 29).expect("Valid date"),
    );

    // Golden Week
    add_with_substitute(
        &mut holidays,
        NaiveDate::from_ymd_opt(year, 5, 3).expect("Valid date"),
    );
    add_with_substitute(
        &mut holidays,
        NaiveDate::from_ymd_opt(year, 5, 5).expect("Valid date"),
    );

    // Apply Golden Week bridge rules (includes May 4)
    apply_golden_week_rules(year, &mut holidays);

    // Marine Day (3rd Monday of July)
    if let Some(date) = nth_weekday_of_month(year, 7, Weekday::Mon, 3) {
        holidays.insert(date);
    }

    // Mountain Day (Aug 11)
    add_with_substitute(
        &mut holidays,
        NaiveDate::from_ymd_opt(year, 8, 11).expect("Valid date"),
    );

    // Respect for Aged Day (3rd Monday of September)
    if let Some(date) = nth_weekday_of_month(year, 9, Weekday::Mon, 3) {
        holidays.insert(date);
    }

    // Autumnal Equinox (around Sept 22-24)
    if let Some(date) = calculate_autumnal_equinox(year) {
        add_with_substitute(&mut holidays, date);
    }

    // Health and Sports Day (2nd Monday of October)
    if let Some(date) = nth_weekday_of_month(year, 10, Weekday::Mon, 2) {
        holidays.insert(date);
    }

    // Culture Day (Nov 3)
    add_with_substitute(
        &mut holidays,
        NaiveDate::from_ymd_opt(year, 11, 3).expect("Valid date"),
    );

    // Labour Thanksgiving Day (Nov 23)
    add_with_substitute(
        &mut holidays,
        NaiveDate::from_ymd_opt(year, 11, 23).expect("Valid date"),
    );

    // Market closes Dec 31
    holidays.insert(NaiveDate::from_ymd_opt(year, 12, 31).expect("Valid date"));

    holidays
}

/// Add holiday with substitute if it falls on Sunday
fn add_with_substitute(holidays: &mut HashSet<NaiveDate>, date: NaiveDate) {
    holidays.insert(date);

    if date.weekday() == Weekday::Sun {
        let mut substitute = date + chrono::Duration::days(1);
        let mut attempts = 0;
        while holidays.contains(&substitute) && attempts < 7 {
            substitute += chrono::Duration::days(1);
            attempts += 1;
        }
        if attempts < 7 {
            holidays.insert(substitute);
        }
    }
}

/// Apply Golden Week bridge day rules
fn apply_golden_week_rules(year: i32, holidays: &mut HashSet<NaiveDate>) {
    let may_3 = NaiveDate::from_ymd_opt(year, 5, 3).expect("Valid date");
    let may_4 = NaiveDate::from_ymd_opt(year, 5, 4).expect("Valid date");
    let may_5 = NaiveDate::from_ymd_opt(year, 5, 5).expect("Valid date");

    // May 4 is always a holiday (Greenery Day)
    holidays.insert(may_4);

    // Bridge day rules
    if may_3.weekday() == Weekday::Tue {
        // May 3 is Tuesday, add May 2 as bridge
        holidays.insert(NaiveDate::from_ymd_opt(year, 5, 2).expect("Valid date"));
    }
    if may_5.weekday() == Weekday::Thu {
        // May 5 is Thursday, add May 6 as bridge
        holidays.insert(NaiveDate::from_ymd_opt(year, 5, 6).expect("Valid date"));
    }
    if may_3.weekday() == Weekday::Fri && may_5.weekday() == Weekday::Sun {
        // May 3 is Friday, May 5 is Sunday, add May 6 as bridge
        holidays.insert(NaiveDate::from_ymd_opt(year, 5, 6).expect("Valid date"));
    }

    // Additional bridge day when May 4 falls on Sunday
    if may_4.weekday() == Weekday::Sun {
        // May 4 is Sunday, add May 6 as substitute
        holidays.insert(NaiveDate::from_ymd_opt(year, 5, 6).expect("Valid date"));
    }
}

/// Calculate Vernal Equinox
fn calculate_vernal_equinox(year: i32) -> Option<NaiveDate> {
    // Official Japanese government equinox dates 2020-2030
    let day = match year {
        2020 => 20,
        2021 => 20,
        2022 => 21,
        2023 => 21,
        2024 => 20,
        2025 => 20,
        2026 => 20,
        2027 => 21,
        2028 => 20,
        2029 => 20,
        2030 => 20,
        _ => return None,
    };
    NaiveDate::from_ymd_opt(year, 3, day)
}

/// Calculate Autumnal Equinox
fn calculate_autumnal_equinox(year: i32) -> Option<NaiveDate> {
    // Official Japanese government equinox dates 2020-2030
    let day = match year {
        2020 => 22,
        2021 => 23,
        2022 => 23,
        2023 => 23,
        2024 => 22,
        2025 => 23,
        2026 => 23,
        2027 => 23,
        2028 => 22,
        2029 => 23,
        2030 => 23,
        _ => return None,
    };
    NaiveDate::from_ymd_opt(year, 9, day)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_japan_holidays_2025() {
        let holidays = get_japan_holidays(2025);

        // New Year holidays
        assert!(holidays.contains(&NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()));
        assert!(holidays.contains(&NaiveDate::from_ymd_opt(2025, 1, 2).unwrap()));
        assert!(holidays.contains(&NaiveDate::from_ymd_opt(2025, 1, 3).unwrap()));

        // Coming of Age Day 2025 (Jan 13)
        assert!(holidays.contains(&NaiveDate::from_ymd_opt(2025, 1, 13).unwrap()));

        // National Foundation Day 2025 (Feb 11)
        assert!(holidays.contains(&NaiveDate::from_ymd_opt(2025, 2, 11).unwrap()));

        // Emperor's Birthday 2025 (Feb 23) - falls on Sunday, so substitute is Feb 24
        assert!(holidays.contains(&NaiveDate::from_ymd_opt(2025, 2, 24).unwrap()));

        // Vernal Equinox 2025 (March 20) - official Japanese date for 2025
        assert!(holidays.contains(&NaiveDate::from_ymd_opt(2025, 3, 20).unwrap()));

        // Showa Day 2025 (Apr 29)
        assert!(holidays.contains(&NaiveDate::from_ymd_opt(2025, 4, 29).unwrap()));

        // Golden Week 2025
        assert!(holidays.contains(&NaiveDate::from_ymd_opt(2025, 5, 3).unwrap())); // Constitution Day
        assert!(holidays.contains(&NaiveDate::from_ymd_opt(2025, 5, 4).unwrap())); // Greenery Day
        assert!(holidays.contains(&NaiveDate::from_ymd_opt(2025, 5, 5).unwrap())); // Children's Day

        // Marine Day 2025 (July 21)
        assert!(holidays.contains(&NaiveDate::from_ymd_opt(2025, 7, 21).unwrap()));

        // Mountain Day 2025 (Aug 11)
        assert!(holidays.contains(&NaiveDate::from_ymd_opt(2025, 8, 11).unwrap()));

        // Respect for Aged Day 2025 (Sept 15)
        assert!(holidays.contains(&NaiveDate::from_ymd_opt(2025, 9, 15).unwrap()));

        // Autumnal Equinox 2025 (Sept 23) - official Japanese date for 2025
        assert!(holidays.contains(&NaiveDate::from_ymd_opt(2025, 9, 23).unwrap()));

        // Health and Sports Day 2025 (Oct 13)
        assert!(holidays.contains(&NaiveDate::from_ymd_opt(2025, 10, 13).unwrap()));

        // Culture Day 2025 (Nov 3)
        assert!(holidays.contains(&NaiveDate::from_ymd_opt(2025, 11, 3).unwrap()));

        // Labour Thanksgiving Day 2025 (Nov 24) - falls on Sunday, so substitute is Nov 24
        assert!(holidays.contains(&NaiveDate::from_ymd_opt(2025, 11, 24).unwrap()));

        // New Year's Eve
        assert!(holidays.contains(&NaiveDate::from_ymd_opt(2025, 12, 31).unwrap()));
    }

    #[test]
    fn test_substitute_holidays() {
        // Test substitute holiday when holiday falls on Sunday
        let holidays = get_japan_holidays(2024);

        // Culture Day 2024 was Sunday Nov 3, so substitute should be Monday Nov 4
        assert!(holidays.contains(&NaiveDate::from_ymd_opt(2024, 11, 4).unwrap()));
    }

    #[test]
    fn test_golden_week_bridge_days() {
        // Test Golden Week bridge days for a year where they apply
        let holidays = get_japan_holidays(2024);

        // May 3, 2024 was Friday, May 5 was Sunday
        // Should have bridge day on May 6 (Monday)
        assert!(holidays.contains(&NaiveDate::from_ymd_opt(2024, 5, 6).unwrap()));
    }
}
