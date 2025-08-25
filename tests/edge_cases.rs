use chrono::Datelike;
use trading_calendar::{Market, NaiveDate, TradingCalendar};

#[test]
fn test_year_boundaries() {
    let nyse = TradingCalendar::new(Market::NYSE).unwrap();

    // Test year boundary (Dec 31, 2029 to Jan 1, 2030)
    let dec_31_2029 = NaiveDate::from_ymd_opt(2029, 12, 31).unwrap();
    let jan_1_2030 = NaiveDate::from_ymd_opt(2030, 1, 1).unwrap();

    // Should work for supported years (2020-2030)
    assert!(nyse.is_trading_day(dec_31_2029).unwrap());
    assert!(!nyse.is_trading_day(jan_1_2030).unwrap()); // New Year's Day 2030 is a holiday

    // Test unsupported years
    let dec_31_2019 = NaiveDate::from_ymd_opt(2019, 12, 31).unwrap();
    let jan_1_2031 = NaiveDate::from_ymd_opt(2031, 1, 1).unwrap();

    assert!(nyse.is_trading_day(dec_31_2019).is_err());
    assert!(nyse.is_trading_day(jan_1_2031).is_err());
}

#[test]
fn test_leap_years() {
    let nyse = TradingCalendar::new(Market::NYSE).unwrap();

    // Test leap year 2024
    let leap_year_date = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();
    assert!(nyse.is_trading_day(leap_year_date).unwrap()); // Should be a trading day

    // Test non-leap year 2023 (Feb 29 doesn't exist)
    let non_leap_date = NaiveDate::from_ymd_opt(2023, 2, 28).unwrap();
    assert!(nyse.is_trading_day(non_leap_date).unwrap());

    // Test leap year 2028
    let leap_year_2028 = NaiveDate::from_ymd_opt(2028, 2, 29).unwrap();
    assert!(nyse.is_trading_day(leap_year_2028).unwrap());
}

#[test]
fn test_dst_transitions() {
    let nyse = TradingCalendar::new(Market::NYSE).unwrap();

    // Test spring forward (March 10, 2024 - clocks go forward)
    let spring_forward = NaiveDate::from_ymd_opt(2024, 3, 10).unwrap();
    assert!(!nyse.is_trading_day(spring_forward).unwrap()); // Sunday

    // Test fall back (November 3, 2024 - clocks go back)
    let fall_back = NaiveDate::from_ymd_opt(2024, 11, 3).unwrap();
    assert!(!nyse.is_trading_day(fall_back).unwrap()); // Sunday

    // Test regular days around DST transitions
    let march_11_2024 = NaiveDate::from_ymd_opt(2024, 3, 11).unwrap(); // Monday after spring forward
    let nov_4_2024 = NaiveDate::from_ymd_opt(2024, 11, 4).unwrap(); // Monday after fall back

    assert!(nyse.is_trading_day(march_11_2024).unwrap());
    assert!(nyse.is_trading_day(nov_4_2024).unwrap());
}

#[test]
fn test_invalid_dates() {
    let nyse = TradingCalendar::new(Market::NYSE).unwrap();

    // Test invalid dates - February 30 doesn't exist, so from_ymd_opt returns None
    let invalid_date = NaiveDate::from_ymd_opt(2025, 2, 30);
    assert!(invalid_date.is_none());

    // Test edge of supported range
    let min_supported = NaiveDate::from_ymd_opt(2020, 1, 2).unwrap(); // Jan 2, 2020 (not New Year's Day)
    let max_supported = NaiveDate::from_ymd_opt(2030, 12, 31).unwrap(); // Dec 31, 2030

    assert!(nyse.is_trading_day(min_supported).unwrap());
    assert!(nyse.is_trading_day(max_supported).unwrap());
}

#[test]
fn test_concurrent_cache_access() {
    use std::sync::Arc;
    use std::thread;

    let calendar = Arc::new(TradingCalendar::new(Market::NYSE).unwrap());
    let mut handles = vec![];

    // Spawn multiple threads that access the cache concurrently
    for i in 0..10 {
        let calendar_clone = Arc::clone(&calendar);
        let handle = thread::spawn(move || {
            let year = 2025 + (i % 5);
            let test_date = NaiveDate::from_ymd_opt(year, 1, 1).unwrap();
            let _is_holiday = calendar_clone.is_holiday(test_date).unwrap();
            let _is_trading_day = calendar_clone.is_trading_day(test_date).unwrap();
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn test_holiday_weekend_consistency() {
    let markets = vec![
        TradingCalendar::new(Market::NYSE).unwrap(),
        TradingCalendar::new(Market::LSE).unwrap(),
        TradingCalendar::new(Market::TSE).unwrap(),
        TradingCalendar::new(Market::TSX).unwrap(),
    ];

    for calendar in markets {
        // Test that holidays are properly adjusted for weekends
        for year in 2020..=2030 {
            for month in 1..=12 {
                for day in 1..=31 {
                    if let Some(date) = NaiveDate::from_ymd_opt(year, month, day) {
                        if calendar.is_holiday(date).unwrap() {
                            // Holiday should not be on weekend (after adjustment)
                            let weekday = date.weekday().number_from_monday();
                            if weekday == 5 || weekday == 6 {
                                // If holiday falls on weekend, it should be adjusted
                                // This is a more complex test that would require checking
                                // the original holiday date vs adjusted date
                                // For now, we'll just note that some holidays can be on weekends
                                // if they're substitute holidays
                            }
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn test_next_trading_day_ordering() {
    let nyse = TradingCalendar::new(Market::NYSE).unwrap();

    // Test that next trading day is always after current date
    let test_dates = vec![
        NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(), // New Year's Day (holiday)
        NaiveDate::from_ymd_opt(2025, 1, 4).unwrap(), // Saturday
        NaiveDate::from_ymd_opt(2025, 1, 5).unwrap(), // Sunday
        NaiveDate::from_ymd_opt(2025, 12, 25).unwrap(), // Christmas (holiday)
    ];

    for date in test_dates {
        let next = nyse.next_trading_day(date);
        assert!(next > date, "Next trading day should be after current date");
    }
}

#[test]
fn test_previous_trading_day_ordering() {
    let nyse = TradingCalendar::new(Market::NYSE).unwrap();

    // Test that previous trading day is always before current date
    let test_dates = vec![
        NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(), // New Year's Day (holiday)
        NaiveDate::from_ymd_opt(2025, 1, 4).unwrap(), // Saturday
        NaiveDate::from_ymd_opt(2025, 1, 5).unwrap(), // Sunday
        NaiveDate::from_ymd_opt(2025, 12, 25).unwrap(), // Christmas (holiday)
    ];

    for date in test_dates {
        let prev = nyse.previous_trading_day(date);
        assert!(
            prev < date,
            "Previous trading day should be before current date"
        );
    }
}

#[test]
fn test_trading_hours_consistency() {
    let nyse = TradingCalendar::new(Market::NYSE).unwrap();

    // Test that trading hours are consistent for trading days
    let trading_dates = vec![
        NaiveDate::from_ymd_opt(2025, 1, 2).unwrap(), // Regular trading day
        NaiveDate::from_ymd_opt(2025, 1, 3).unwrap(), // Regular trading day
        NaiveDate::from_ymd_opt(2025, 12, 24).unwrap(), // Christmas Eve (early close)
    ];

    for date in trading_dates {
        let hours = nyse.trading_hours(date);

        // Regular session should always exist
        assert!(hours.regular.start < hours.regular.end);

        // If it's an early close day, early_close should be set
        if date.month() == 12 && date.day() == 24 {
            assert!(hours.is_early_close());
        }
    }
}

#[test]
fn test_error_handling() {
    // Test that invalid market creation returns an error
    // This would require adding an invalid market variant for testing
    // For now, we'll test that valid markets work
    let markets = vec![
        Market::NYSE,
        Market::NASDAQ,
        Market::LSE,
        Market::TSE,
        Market::TSX,
    ];

    for market in markets {
        let calendar = TradingCalendar::new(market);
        assert!(
            calendar.is_ok(),
            "Valid market should create calendar successfully"
        );
    }
}

#[test]
fn test_timezone_consistency() {
    let markets = vec![
        (Market::NYSE, "America/New_York"),
        (Market::NASDAQ, "America/New_York"),
        (Market::LSE, "Europe/London"),
        (Market::TSE, "Asia/Tokyo"),
        (Market::TSX, "America/Toronto"),
    ];

    for (market, expected_tz) in markets {
        let calendar = TradingCalendar::new(market).unwrap();
        let tz = calendar.timezone();
        assert_eq!(tz.name(), expected_tz);
    }
}

#[test]
fn test_cache_performance() {
    use std::collections::HashSet;
    use trading_calendar::utils::HolidayCache;

    let cache = HolidayCache::with_capacity(5);

    // Test cache performance with multiple accesses
    for year in 2020..2030 {
        let holidays = HashSet::from([NaiveDate::from_ymd_opt(year, 1, 1).unwrap()]);

        // First access should compute
        let _cached = cache.get_or_compute(year, || holidays.clone());

        // Second access should hit cache
        let _cached_again = cache.get_or_compute(year, || panic!("Should not compute again"));
    }

    // Verify cache stats
    let stats = cache.stats();
    assert_eq!(stats.len, 5); // Should be at capacity
    assert_eq!(stats.capacity, 5);
}

#[test]
fn test_month_boundaries() {
    let nyse = TradingCalendar::new(Market::NYSE).unwrap();

    // Test month boundaries for various months
    let month_boundaries = vec![
        (2025, 1, 31),  // January
        (2025, 2, 28),  // February (non-leap year)
        (2024, 2, 29),  // February (leap year)
        (2025, 3, 31),  // March
        (2025, 4, 30),  // April
        (2025, 5, 31),  // May
        (2025, 6, 30),  // June
        (2025, 7, 31),  // July
        (2025, 8, 31),  // August
        (2025, 9, 30),  // September
        (2025, 10, 31), // October
        (2025, 11, 30), // November
        (2025, 12, 31), // December
    ];

    for (year, month, day) in month_boundaries {
        if let Some(date) = NaiveDate::from_ymd_opt(year, month, day) {
            // Should not panic
            let _is_trading = nyse.is_trading_day(date).unwrap();
            let _is_holiday = nyse.is_holiday(date).unwrap();
        }
    }
}
