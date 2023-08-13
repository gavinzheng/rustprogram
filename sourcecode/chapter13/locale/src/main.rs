use locale::*;
fn main() {
    let mut l = user_locale_factory();

    let numeric_locale = l.get_numeric().unwrap();
    println!(
        "Numbers: decimal sep: {} thousands sep: {}",
        numeric_locale.decimal_sep, numeric_locale.thousands_sep
    );

    let time = l.get_time().unwrap();
    println!("Time:");
    println!(
        "  January: Long: {}, Short: {}",
        time.long_month_name(0),
        time.short_month_name(0)
    );
    println!(
        "  Monday: Long: {}, Short: {}",
        time.long_day_name(1),
        time.short_day_name(1)
    );
  }