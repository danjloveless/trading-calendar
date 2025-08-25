use trading_calendar::{Market, NaiveDate, TradingCalendar};

fn main() -> trading_calendar::Result<()> {
    let nyse = TradingCalendar::new(Market::NYSE)?;

    println!("=== NYSE Trading Calendar Demo ===\n");

    // Check if market is open now
    if nyse.is_open_now()? {
        println!("✅ NYSE is currently OPEN");
        if let Ok(close) = nyse.next_close() {
            println!("   Closes at: {}", close.format("%I:%M %p %Z"));
        }
    } else {
        println!("❌ NYSE is currently CLOSED");
        if let Ok(open) = nyse.next_open() {
            println!("   Opens at: {}", open.format("%A, %B %d at %I:%M %p %Z"));
        }
    }

    println!("\n=== Checking Specific Dates ===\n");

    let dates = vec![
        NaiveDate::from_ymd_opt(2025, 12, 25).unwrap(), // Christmas
        NaiveDate::from_ymd_opt(2025, 12, 24).unwrap(), // Christmas Eve
        NaiveDate::from_ymd_opt(2025, 7, 4).unwrap(),   // Independence Day
        NaiveDate::from_ymd_opt(2025, 7, 3).unwrap(),   // July 3rd
    ];

    for date in dates {
        print!("{}: ", date.format("%B %d, %Y"));

        if nyse.is_trading_day(date)? {
            let hours = nyse.trading_hours(date);
            if hours.is_early_close() {
                println!("Early close at 1:00 PM");
            } else {
                println!("Regular trading day");
            }
        } else {
            println!("Market closed");
        }
    }

    Ok(())
}
