use trading_calendar::{Holiday, Market, NaiveDate, NaiveTime, TradingCalendar};

#[test]
fn test_lse_holidays() {
    let lse = TradingCalendar::new(Market::LSE).unwrap();

    // Boxing Day 2025
    assert!(lse
        .is_holiday(NaiveDate::from_ymd_opt(2025, 12, 26).unwrap())
        .unwrap());

    // Good Friday 2025
    assert!(lse
        .is_holiday(NaiveDate::from_ymd_opt(2025, 4, 18).unwrap())
        .unwrap());
}

#[test]
fn test_tse_holidays() {
    let tse = TradingCalendar::new(Market::TSE).unwrap();

    // New Year holidays
    assert!(tse
        .is_holiday(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap())
        .unwrap());
    assert!(tse
        .is_holiday(NaiveDate::from_ymd_opt(2025, 1, 2).unwrap())
        .unwrap());
    assert!(tse
        .is_holiday(NaiveDate::from_ymd_opt(2025, 1, 3).unwrap())
        .unwrap());
}

#[test]
fn test_tsx_holidays() {
    let tsx = TradingCalendar::new(Market::TSX).unwrap();

    // Canada Day 2025
    assert!(tsx
        .is_holiday(NaiveDate::from_ymd_opt(2025, 7, 1).unwrap())
        .unwrap());

    // Victoria Day 2025
    assert!(tsx
        .is_holiday(NaiveDate::from_ymd_opt(2025, 5, 19).unwrap())
        .unwrap());
}

#[test]
fn test_japanese_equinox_accuracy() {
    // Test accurate equinox dates for Japanese market
    let tse = TradingCalendar::new(Market::TSE).unwrap();

    // Test vernal equinox dates
    assert!(tse
        .is_holiday(NaiveDate::from_ymd_opt(2024, 3, 20).unwrap())
        .unwrap()); // 2024: March 20
    assert!(tse
        .is_holiday(NaiveDate::from_ymd_opt(2025, 3, 20).unwrap())
        .unwrap()); // 2025: March 20
    assert!(tse
        .is_holiday(NaiveDate::from_ymd_opt(2026, 3, 20).unwrap())
        .unwrap()); // 2026: March 20
    assert!(tse
        .is_holiday(NaiveDate::from_ymd_opt(2027, 3, 21).unwrap())
        .unwrap()); // 2027: March 21

    // Test autumnal equinox dates
    assert!(tse
        .is_holiday(NaiveDate::from_ymd_opt(2024, 9, 22).unwrap())
        .unwrap()); // 2024: September 22
    assert!(tse
        .is_holiday(NaiveDate::from_ymd_opt(2025, 9, 23).unwrap())
        .unwrap()); // 2025: September 23
    assert!(tse
        .is_holiday(NaiveDate::from_ymd_opt(2026, 9, 23).unwrap())
        .unwrap()); // 2026: September 23
    assert!(tse
        .is_holiday(NaiveDate::from_ymd_opt(2027, 9, 23).unwrap())
        .unwrap()); // 2027: September 23
}

#[test]
fn test_us_after_hours_on_early_close() {
    let nyse = TradingCalendar::new(Market::NYSE).unwrap();

    // Test Christmas Eve 2025 (early close day)
    let christmas_eve = NaiveDate::from_ymd_opt(2025, 12, 24).unwrap();
    let hours = nyse.trading_hours(christmas_eve);

    assert!(hours.is_early_close());
    assert!(hours.after_hours.is_some());

    let after = hours.after_hours.unwrap();
    // After-hours should start at early close time (13:00) and end at after-hours close (20:00)
    assert_eq!(after.start, NaiveTime::from_hms_opt(13, 0, 0).unwrap());
    assert_eq!(after.end, NaiveTime::from_hms_opt(20, 0, 0).unwrap());
}

#[test]
fn test_year_2030_support() {
    let nyse = TradingCalendar::new(Market::NYSE).unwrap();

    // Test that year 2030 is supported
    let jan_2_2030 = NaiveDate::from_ymd_opt(2030, 1, 2).unwrap(); // Wednesday
    assert!(nyse.is_trading_day(jan_2_2030).unwrap()); // Should be a trading day

    let jan_1_2030 = NaiveDate::from_ymd_opt(2030, 1, 1).unwrap(); // Tuesday
    assert!(!nyse.is_trading_day(jan_1_2030).unwrap()); // New Year's Day holiday

    let dec_31_2030 = NaiveDate::from_ymd_opt(2030, 12, 31).unwrap(); // Tuesday
    assert!(nyse.is_trading_day(dec_31_2030).unwrap()); // Should be a trading day
}

#[test]
fn test_canada_non_market_holidays_removed() {
    let tsx = TradingCalendar::new(Market::TSX).unwrap();

    // Test that Civic Holiday (August) is NOT a holiday for TSX
    let civic_holiday_2025 = NaiveDate::from_ymd_opt(2025, 8, 4).unwrap(); // First Monday of August
    assert!(tsx.is_trading_day(civic_holiday_2025).unwrap()); // TSX should be OPEN

    // Test that Family Day is still a holiday (it is observed by TSX)
    let family_day_2025 = NaiveDate::from_ymd_opt(2025, 2, 17).unwrap(); // Third Monday of February
    assert!(!tsx.is_trading_day(family_day_2025).unwrap()); // TSX should be CLOSED

    // Test that other TSX holidays are still observed
    let canada_day_2025 = NaiveDate::from_ymd_opt(2025, 7, 1).unwrap();
    assert!(!tsx.is_trading_day(canada_day_2025).unwrap()); // Canada Day holiday
}

#[test]
fn test_holiday_struct_usage() {
    // Test that the Holiday struct can be created and used
    let holiday = Holiday::new(
        NaiveDate::from_ymd_opt(2025, 12, 25).unwrap(),
        "Christmas Day",
        true,
    );

    assert_eq!(holiday.date, NaiveDate::from_ymd_opt(2025, 12, 25).unwrap());
    assert_eq!(holiday.name, "Christmas Day");
    assert!(holiday.market_closed);
    assert!(holiday.early_close.is_none());

    // Test holiday with early close
    let early_close_holiday = Holiday::with_early_close(
        NaiveDate::from_ymd_opt(2025, 12, 24).unwrap(),
        "Christmas Eve",
        NaiveTime::from_hms_opt(13, 0, 0).unwrap(),
    );

    assert_eq!(
        early_close_holiday.date,
        NaiveDate::from_ymd_opt(2025, 12, 24).unwrap()
    );
    assert_eq!(early_close_holiday.name, "Christmas Eve");
    assert!(!early_close_holiday.market_closed);
    assert_eq!(
        early_close_holiday.early_close,
        Some(NaiveTime::from_hms_opt(13, 0, 0).unwrap())
    );
}
