use chrono::{Date, Datelike, Duration, Local, Month, NaiveDate};
use num_traits::cast::FromPrimitive;

#[derive(Debug)]
struct Ymd {
    year: i32,
    month: u32,
    day: u32,
}

impl Ymd {
    fn new(string: String) -> Self {
        let mut split = string.split('/');

        match (split.next(), split.next(), split.next()) {
            (Some(m), Some(d), Some(y)) => {
                let year = Self::validate_year(y);
                let month = Self::validate_month(m);
                let day = Self::validate_day(month, d, is_leap_year(year));

                Self { year, month, day }
            }
            (Some(m), Some(d), None) => {
                let year = Local::today().year();
                let month = Self::validate_month(m);
                let day = Self::validate_day(month, d, is_leap_year(year));

                Self { year, month, day }
            }
            _ => {
                eprintln!("The given input might be incorrectly formatted.\nUSAGE: cargo run mm/dd/yyyy // year input defaults to the current year if omitted");
                std::process::exit(1);
            }
        }
    }

    fn validate_year(y: &str) -> i32 {
        y.parse::<i32>().expect("Number expected.")
    }

    fn validate_month(m: &str) -> u32 {
        let month = m.parse::<u32>().expect("Number expected.");
        match month {
            (1..=12) => month,
            _ => {
                eprintln!("The given month does not exist.");
                std::process::exit(1);
            }
        }
    }

    fn validate_day(month: u32, d: &str, is_leap_year: bool) -> u32 {
        let day = d.parse::<u32>().expect("Number expected.");
        match (month, day, is_leap_year) {
            (1 | 3 | 5 | 7 | 8 | 10 | 12, 1..=31, _) => day,
            (4 | 6 | 9 | 11, 1..=30, _) => day,
            (2, 1..=29, true) => day,
            (2, 1..=28, false) => day,
            _ => {
                eprintln!(
                    "The given day does not exist in the month of {:?}.",
                    Month::from_u32(month).unwrap()
                );
                std::process::exit(1);
            }
        }
    }
}

impl From<Ymd> for NaiveDate {
    fn from(ymd: Ymd) -> Self {
        NaiveDate::from_ymd(ymd.year, ymd.month, ymd.day)
    }
}

fn is_leap_year(year: i32) -> bool {
    year % 400 == 0 || (year % 4 == 0 && year % 100 != 0)
}

fn count_days(date: NaiveDate) -> Duration {
    date - Date::naive_local(&Local::today())
}

fn print_days_diff(days_diff: Duration, date: NaiveDate) {
    if days_diff > Duration::zero() {
        println!(
            "{} days left until {}, {}.",
            days_diff.num_days(),
            date,
            date.weekday()
        );
    } else {
        println!(
            "{} days has passed since {}, {}.",
            -days_diff.num_days(),
            date,
            date.weekday()
        );
    }
}

fn main() {
    let mut args = std::env::args().skip(1);

    if args.len() != 1 {
        eprintln!(
            "Wrong number of inputs; expected 1, but given {}.\nUSAGE: cargo run mm/dd/yyyy // year input defaults to the current year if omitted",
            args.len()
        );
        std::process::exit(1);
    }

    let date = Ymd::new(args.next().unwrap()).into();
    let days_diff = count_days(date);

    print_days_diff(days_diff, date);
}
