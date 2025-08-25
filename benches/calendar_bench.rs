use criterion::{black_box, criterion_group, criterion_main, Criterion};
use trading_calendar::{Market, NaiveDate, TradingCalendar};

fn benchmark_is_trading_day(c: &mut Criterion) {
    let calendar = TradingCalendar::new(Market::NYSE).expect("Failed to create calendar");

    c.bench_function("is_trading_day", |b| {
        let date = NaiveDate::from_ymd_opt(2025, 7, 15).unwrap();
        b.iter(|| calendar.is_trading_day(black_box(date)));
    });
}

fn benchmark_trading_hours(c: &mut Criterion) {
    let calendar = TradingCalendar::new(Market::NYSE).expect("Failed to create calendar");

    c.bench_function("trading_hours", |b| {
        let date = NaiveDate::from_ymd_opt(2025, 7, 15).unwrap();
        b.iter(|| calendar.trading_hours(black_box(date)));
    });
}

criterion_group!(benches, benchmark_is_trading_day, benchmark_trading_hours);
criterion_main!(benches);
