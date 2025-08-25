use trading_calendar::{Market, NaiveDate, TradingCalendar};

fn main() -> trading_calendar::Result<()> {
    let nyse = TradingCalendar::new(Market::NYSE)?;

    println!("NYSE Holidays for 2025\n");
    println!("{:<30} {}", "Holiday", "Date");
    println!("{:-<50}", "");

    let test_dates = vec![
        (
            "New Year's Day",
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
        ),
        ("MLK Day", NaiveDate::from_ymd_opt(2025, 1, 20).unwrap()),
        (
            "Presidents Day",
            NaiveDate::from_ymd_opt(2025, 2, 17).unwrap(),
        ),
        ("Good Friday", NaiveDate::from_ymd_opt(2025, 4, 18).unwrap()),
        (
            "Memorial Day",
            NaiveDate::from_ymd_opt(2025, 5, 26).unwrap(),
        ),
        ("Juneteenth", NaiveDate::from_ymd_opt(2025, 6, 19).unwrap()),
        (
            "Independence Day",
            NaiveDate::from_ymd_opt(2025, 7, 4).unwrap(),
        ),
        ("Labor Day", NaiveDate::from_ymd_opt(2025, 9, 1).unwrap()),
        (
            "Thanksgiving",
            NaiveDate::from_ymd_opt(2025, 11, 27).unwrap(),
        ),
        ("Christmas", NaiveDate::from_ymd_opt(2025, 12, 25).unwrap()),
    ];

    for (name, date) in test_dates {
        let status = if nyse.is_holiday(date)? {
            format!("{} ✓", date.format("%b %d (%a)"))
        } else {
            "Not a holiday".to_string()
        };
        println!("{:<30} {}", name, status);
    }

    Ok(())
}
