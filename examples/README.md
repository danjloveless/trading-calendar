# Examples

This directory contains examples of using the trading-calendar crate.

## Running Examples

```bash
# Run a specific example
cargo run --example basic_usage

# Run all examples
for example in basic_usage check_holidays holiday_info; do
    cargo run --example $example
done
```

## Available Examples

- **basic_usage**: Simple calendar operations and checking if markets are open
- **check_holidays**: List all holidays for a given year  
- **holiday_info**: Work with the Holiday struct for detailed information

## Example Descriptions

### basic_usage.rs
Demonstrates basic usage including:
- Checking if market is currently open
- Finding next market open/close times
- Checking specific dates for trading days vs holidays
- Identifying early close days

### check_holidays.rs
Shows how to:
- List all market holidays for a year
- Verify specific holiday dates
- Check holiday status for multiple dates

### holiday_info.rs
Illustrates:
- Creating Holiday structs manually
- Working with early close information
- Checking after-hours availability on early close days
