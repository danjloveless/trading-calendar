//! Utility functions for calendar operations

pub mod cache;
pub mod easter;

pub use cache::HolidayCache;
pub use easter::{calculate_easter_monday, calculate_good_friday};

use chrono::{Datelike, NaiveDate, Weekday};

/// Calculate the nth occurrence of a weekday in a month
pub fn nth_weekday_of_month(year: i32, month: u32, weekday: Weekday, nth: u8) -> Option<NaiveDate> {
    let first_day = NaiveDate::from_ymd_opt(year, month, 1)?;
    let first_weekday = first_day.weekday();

    let days_to_add =
        (weekday.num_days_from_monday() as i32 - first_weekday.num_days_from_monday() as i32 + 7)
            % 7;
    let first_occurrence = first_day + chrono::Duration::days(days_to_add.into());

    let target_date = first_occurrence + chrono::Duration::weeks((nth - 1) as i64);

    if target_date.month() == month {
        Some(target_date)
    } else {
        None
    }
}

/// Calculate the last occurrence of a weekday in a month
pub fn last_weekday_of_month(year: i32, month: u32, weekday: Weekday) -> Option<NaiveDate> {
    let next_month = if month == 12 { 1 } else { month + 1 };
    let next_year = if month == 12 { year + 1 } else { year };

    let first_of_next = NaiveDate::from_ymd_opt(next_year, next_month, 1)?;
    let mut last_day = first_of_next - chrono::Duration::days(1);

    while last_day.weekday() != weekday {
        last_day -= chrono::Duration::days(1);
    }

    Some(last_day)
}
