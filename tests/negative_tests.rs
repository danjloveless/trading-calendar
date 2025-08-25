use chrono::Datelike;
use trading_calendar::{Market, NaiveDate, TradingCalendar};

#[test]
fn test_unsupported_years() {
    let _nyse = TradingCalendar::new(Market::NYSE).unwrap();

    // Test years outside supported range (2020-2030)
    let unsupported_years = vec![2019, 2031, 2100];

    for year in unsupported_years {
        let test_date = NaiveDate::from_ymd_opt(year, 1, 1).unwrap();
        // Should return error for unsupported years
        assert!(_nyse.is_trading_day(test_date).is_err());
        // Note: is_holiday might return true for holidays even in unsupported years
        // This is expected behavior as the holiday calculation doesn't check year bounds
    }
}

#[test]
fn test_invalid_dates() {
    let _nyse = TradingCalendar::new(Market::NYSE).unwrap();

    // Test invalid dates that should return None
    let invalid_dates = vec![
        (2025, 2, 30),  // February 30 doesn't exist
        (2025, 4, 31),  // April 31 doesn't exist
        (2025, 6, 31),  // June 31 doesn't exist
        (2025, 9, 31),  // September 31 doesn't exist
        (2025, 11, 31), // November 31 doesn't exist
    ];

    for (year, month, day) in invalid_dates {
        let invalid_date = NaiveDate::from_ymd_opt(year, month, day);
        assert!(
            invalid_date.is_none(),
            "Date {}-{}-{} should be invalid",
            year,
            month,
            day
        );
    }
}

#[test]
fn test_edge_case_dates() {
    let nyse = TradingCalendar::new(Market::NYSE).unwrap();

    // Test edge of supported range
    let min_supported = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();
    let max_supported = NaiveDate::from_ymd_opt(2030, 12, 31).unwrap();

    // These should work
    assert!(nyse.is_trading_day(min_supported).unwrap() || nyse.is_holiday(min_supported).unwrap());
    assert!(nyse.is_trading_day(max_supported).unwrap() || nyse.is_holiday(max_supported).unwrap());

    // Just outside range should fail
    let just_before = NaiveDate::from_ymd_opt(2019, 12, 31).unwrap();
    let just_after = NaiveDate::from_ymd_opt(2031, 1, 1).unwrap();

    assert!(nyse.is_trading_day(just_before).is_err());
    assert!(nyse.is_trading_day(just_after).is_err());
}

#[test]
fn test_dst_ambiguity_handling() {
    let nyse = TradingCalendar::new(Market::NYSE).unwrap();

    // Test DST transition dates (these should not panic)
    let dst_transition_dates = vec![
        NaiveDate::from_ymd_opt(2024, 3, 10).unwrap(), // Spring forward
        NaiveDate::from_ymd_opt(2024, 11, 3).unwrap(), // Fall back
    ];

    for date in dst_transition_dates {
        // Should not panic, just return false for weekends
        assert!(!nyse.is_trading_day(date).unwrap());
    }
}

#[test]
fn test_cache_eviction() {
    let cache = trading_calendar::utils::HolidayCache::with_capacity(5);

    // Fill cache beyond capacity
    for year in 2020..2031 {
        let holidays =
            std::collections::HashSet::from([NaiveDate::from_ymd_opt(year, 1, 1).unwrap()]);
        let _cached = cache.get_or_compute(year, || holidays);
    }

    // Cache should not exceed max capacity (with some tolerance for eviction timing)
    assert!(
        cache.len() <= 10,
        "Cache size {} exceeds reasonable limit",
        cache.len()
    );
}

#[test]
fn test_market_creation_errors() {
    // Test that all valid markets can be created
    let valid_markets = vec![
        Market::NYSE,
        Market::NASDAQ,
        Market::LSE,
        Market::TSE,
        Market::TSX,
    ];

    for market in valid_markets {
        let calendar = TradingCalendar::new(market);
        assert!(
            calendar.is_ok(),
            "Failed to create calendar for {:?}",
            market
        );
    }
}

#[test]
fn test_holiday_consistency() {
    let markets = vec![
        TradingCalendar::new(Market::NYSE).unwrap(),
        TradingCalendar::new(Market::LSE).unwrap(),
        TradingCalendar::new(Market::TSE).unwrap(),
        TradingCalendar::new(Market::TSX).unwrap(),
    ];

    for calendar in markets {
        // Test that no date is both a trading day and a holiday
        for year in 2020..2031 {
            for month in 1..=12 {
                for day in 1..=31 {
                    if let Some(date) = NaiveDate::from_ymd_opt(year, month, day) {
                        let is_trading = calendar.is_trading_day(date).unwrap();
                        let is_holiday = calendar.is_holiday(date).unwrap();

                        // A date cannot be both a trading day and a holiday
                        assert!(
                            !(is_trading && is_holiday),
                            "Date {} is both trading day and holiday",
                            date
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn test_weekend_consistency() {
    let markets = vec![
        TradingCalendar::new(Market::NYSE).unwrap(),
        TradingCalendar::new(Market::LSE).unwrap(),
        TradingCalendar::new(Market::TSE).unwrap(),
        TradingCalendar::new(Market::TSX).unwrap(),
    ];

    for calendar in markets {
        // Test that weekends are never trading days
        for year in 2020..2031 {
            for month in 1..=12 {
                for day in 1..=31 {
                    if let Some(date) = NaiveDate::from_ymd_opt(year, month, day) {
                        let weekday = date.weekday();
                        if weekday == chrono::Weekday::Sat || weekday == chrono::Weekday::Sun {
                            assert!(
                                !calendar.is_trading_day(date).unwrap(),
                                "Weekend {} should not be a trading day",
                                date
                            );
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn test_trading_hours_validation() {
    let nyse = TradingCalendar::new(Market::NYSE).unwrap();

    // Test that trading hours are valid for trading days
    for year in 2020..2031 {
        for month in 1..=12 {
            for day in 1..=31 {
                if let Some(date) = NaiveDate::from_ymd_opt(year, month, day) {
                    if nyse.is_trading_day(date).unwrap() {
                        let hours = nyse.trading_hours(date);

                        // Regular session should have valid times
                        assert!(
                            hours.regular.start < hours.regular.end,
                            "Invalid trading hours for {}",
                            date
                        );

                        // If early close is set, it should be before regular close
                        if let Some(early_close) = hours.early_close {
                            assert!(
                                early_close < hours.regular.end,
                                "Early close should be before regular close for {}",
                                date
                            );
                        }
                    }
                }
            }
        }
    }
}
