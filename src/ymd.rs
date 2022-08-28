use chrono::{Date, Datelike, Duration, Local, Month, NaiveDate};
use num_traits::cast::FromPrimitive;

enum YmdType {
    Year,
    Month,
    Day,
}

#[derive(Debug)]
pub struct Ymd {
    year: i32,
    month: u32,
    day: u32,
}

impl Ymd {
    pub fn new(string: String) -> Self {
        let mut split = string.split('/');

        match (split.next(), split.next(), split.next()) {
            (Some(m), Some(d), Some(y)) => {
                let year = Self::validate_year(y);
                let month = Self::validate_month(m);
                let day = Self::validate_day(month, d, is_leap_year(year));

                Self { year, month, day }
            }
            (Some(m), Some(d), None) => {
                let month = Self::validate_month(m);
                let day = Self::validate_day(month, d, false);
                let year = Self::handle_empty_year(month, day);

                Self { year, month, day }
            }
            _ => {
                eprintln!("The given input might be incorrectly formatted.\nUSAGE: cargo run mm/dd/yyyy // you can optionally omit year input");
                std::process::exit(1);
            }
        }
    }

    fn validate_year(y: &str) -> i32 {
        Self::parse_num(y, YmdType::Year) as i32
    }

    fn validate_month(m: &str) -> u32 {
        let month = Self::parse_num(m, YmdType::Month);
        match month {
            (1..=12) => month,
            _ => {
                eprintln!("The given month does not exist.");
                std::process::exit(1);
            }
        }
    }

    fn validate_day(month: u32, d: &str, is_leap_year: bool) -> u32 {
        let day = Self::parse_num(d, YmdType::Day);
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

    fn parse_num(n: &str, ymd_ty: YmdType) -> u32 {
        match n.parse::<u32>() {
            Ok(num) => num,
            _ => {
                let ty = match ymd_ty {
                    YmdType::Year => "ParseYearError",
                    YmdType::Month => "ParseMonthError",
                    YmdType::Day => "ParseDayError",
                };
                eprintln!("{}: Expected a number; given \"{}\".", ty, n);
                std::process::exit(1);
            }
        }
    }

    fn handle_empty_year(month: u32, day: u32) -> i32 {
        let today = Local::today();
        let current_month = today.month();
        let current_day = today.day();
        let mut year = today.year();

        match (
            month < current_month,
            month == current_month,
            day < current_day,
        ) {
            (true, _, _) => year += 1,
            (false, true, true) => year += 1,
            _ => {}
        }

        if let (true, false, true) = (month == 2, is_leap_year(year), day == 29) {
            eprintln!(
                "The given day does not exist in the month of {:?}.",
                Month::from_u32(month).unwrap()
            );
            std::process::exit(1);
        }

        year
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

pub fn count_days(date: NaiveDate) -> Duration {
    date - Date::naive_local(&Local::today())
}

pub fn print_days_diff(days_diff: Duration, date: NaiveDate) {
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
