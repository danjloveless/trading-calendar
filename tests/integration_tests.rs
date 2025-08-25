use trading_calendar::{Market, NaiveDate, TradingCalendar};

#[test]
fn test_nyse_holidays_2025() {
    let cal = TradingCalendar::new(Market::NYSE).expect("Failed to create NYSE calendar");

    // Test major holidays
    assert!(!cal
        .is_trading_day(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap())
        .unwrap());
    assert!(!cal
        .is_trading_day(NaiveDate::from_ymd_opt(2025, 7, 4).unwrap())
        .unwrap());
    assert!(!cal
        .is_trading_day(NaiveDate::from_ymd_opt(2025, 12, 25).unwrap())
        .unwrap());
}

#[test]
fn test_early_closes() {
    let cal = TradingCalendar::new(Market::NYSE).expect("Failed to create NYSE calendar");

    // Test early close days
    let july_3 = NaiveDate::from_ymd_opt(2025, 7, 3).unwrap();
    let hours = cal.trading_hours(july_3);
    assert!(hours.is_early_close());

    let black_friday = NaiveDate::from_ymd_opt(2025, 11, 28).unwrap();
    let hours = cal.trading_hours(black_friday);
    assert!(hours.is_early_close());
}

#[test]
fn test_weekends() {
    let cal = TradingCalendar::new(Market::NYSE).expect("Failed to create NYSE calendar");

    // Saturday and Sunday should not be trading days
    assert!(!cal
        .is_trading_day(NaiveDate::from_ymd_opt(2025, 8, 23).unwrap())
        .unwrap());
    assert!(!cal
        .is_trading_day(NaiveDate::from_ymd_opt(2025, 8, 24).unwrap())
        .unwrap());
}

#[test]
fn test_thread_safety() {
    use std::sync::Arc;
    use std::thread;

    let calendar = Arc::new(TradingCalendar::new(Market::NYSE).expect("Failed to create calendar"));
    let mut handles = vec![];

    for i in 0..5 {
        let cal = Arc::clone(&calendar);
        let handle = thread::spawn(move || {
            let date = NaiveDate::from_ymd_opt(2025, 1, 2 + i).unwrap();
            let _is_trading = cal.is_trading_day(date).unwrap();
            let _hours = cal.trading_hours(date);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
