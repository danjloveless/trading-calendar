//! Example: Working with Holiday information
//!
//! This example demonstrates how to use the Holiday struct to get detailed
//! information about market holidays.

use trading_calendar::{Holiday, Market, NaiveDate, NaiveTime, TradingCalendar};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let nyse = TradingCalendar::new(Market::NYSE)?;

    // Example: Create a holiday manually
    let christmas = Holiday::new(
        NaiveDate::from_ymd_opt(2025, 12, 25).unwrap(),
        "Christmas Day",
        true,
    );

    println!("Holiday: {} on {}", christmas.name, christmas.date);
    println!("Market closed: {}", christmas.market_closed);

    // Example: Create a holiday with early close
    let christmas_eve = Holiday::with_early_close(
        NaiveDate::from_ymd_opt(2025, 12, 24).unwrap(),
        "Christmas Eve",
        NaiveTime::from_hms_opt(13, 0, 0).unwrap(),
    );

    println!("Holiday: {} on {}", christmas_eve.name, christmas_eve.date);
    println!("Early close: {:?}", christmas_eve.early_close);

    // Example: Check if specific dates are holidays
    let test_dates = vec![
        NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(), // New Year's Day
        NaiveDate::from_ymd_opt(2025, 1, 2).unwrap(), // Regular trading day
        NaiveDate::from_ymd_opt(2025, 12, 25).unwrap(), // Christmas
        NaiveDate::from_ymd_opt(2025, 12, 24).unwrap(), // Christmas Eve
    ];

    for date in test_dates {
        let is_holiday = nyse.is_holiday(date)?;
        let is_trading = nyse.is_trading_day(date)?;
        let hours = nyse.trading_hours(date);

        println!(
            "{}: Holiday={}, Trading={}, Early Close={:?}",
            date, is_holiday, is_trading, hours.early_close
        );
    }

    // Example: Check early close days
    let christmas_eve_date = NaiveDate::from_ymd_opt(2025, 12, 24).unwrap();
    let hours = nyse.trading_hours(christmas_eve_date);

    if hours.is_early_close() {
        println!(
            "Christmas Eve 2025: Early close at {}",
            hours.early_close.unwrap().format("%H:%M")
        );

        // Check if after-hours trading is available
        if let Some(after_hours) = hours.after_hours {
            println!(
                "After-hours trading: {} - {}",
                after_hours.start.format("%H:%M"),
                after_hours.end.format("%H:%M")
            );
        }
    }

    Ok(())
}
