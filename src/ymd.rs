use std::fmt;

use chrono::{Date, Datelike, Duration, Local, Month, NaiveDate};
use num_traits::FromPrimitive;

enum YmdError {
    WrongFormat,
    NaN(YmdType, String),
    InvalidNumber(ValidationError),
}

impl fmt::Debug for YmdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            YmdError::WrongFormat => write!(f, "WrongFormat: The given input might be incorrectly formatted.\nUSAGE: cargo run mm/dd/yyyy // you can optionally omit year input"),
            YmdError::NaN(ymd_ty, n) => {
                let ty = match ymd_ty {
                    YmdType::Year => "ParseYearError",
                    YmdType::Month => "ParseMonthError",
                    YmdType::Day => "ParseDayError",
                };
                write!(f, "{}: Expected a number; given \"{}\".", ty, n)
            }
            YmdError::InvalidNumber(error) => match error {
                ValidationError::MonthDoesNotExist => write!(f, "ValidationError: The given month does not exist."),
                ValidationError::DayDoesNotExist(month, year) => match month {
                    Month::February => write!(f, "ValidationError: The given day does not exist in the month of {:?}, {}.", month, year.unwrap()),
                    _ => write!(f, "ValidationError: The given day does not exist in the month of {:?}.", month)
                },
            },
        }
    }
}

#[derive(Debug)]
enum ValidationError {
    MonthDoesNotExist,
    DayDoesNotExist(Month, Option<i32>),
}

#[derive(Debug)]
enum YmdType {
    Year,
    Month,
    Day,
}

#[derive(Debug, Default)]
pub struct Ymd {
    year: i32,
    month: u32,
    day: u32,
}

impl Ymd {
    pub fn new(input: String) -> Self {
        let mut ymd = Self::default();
        if let Err(error) = ymd.parse_input(input) {
            eprintln!("{:?}", error);
            std::process::exit(1);
        }
        ymd
    }

    fn parse_input(&mut self, input: String) -> Result<(), YmdError> {
        let mut split = input.split('/');
        match (split.next(), split.next(), split.next()) {
            (Some(m), Some(d), Some(y)) => {
                self.parse_year(y)?;
                self.parse_month(m)?;
                self.parse_day(d)?;
                Ok(())
            }
            (Some(m), Some(d), None) => {
                self.parse_month(m)?;
                self.parse_day(d)?;
                self.handle_missing_year()?;
                Ok(())
            }
            _ => Err(YmdError::WrongFormat),
        }
    }

    fn parse_year(&mut self, y: &str) -> Result<(), YmdError> {
        self.parse_num(y, YmdType::Year)
    }

    fn parse_month(&mut self, m: &str) -> Result<(), YmdError> {
        self.parse_num(m, YmdType::Month)
    }

    fn parse_day(&mut self, d: &str) -> Result<(), YmdError> {
        self.parse_num(d, YmdType::Day)
    }

    fn validate_month(&self) -> Result<(), YmdError> {
        match self.month {
            (1..=12) => Ok(()),
            _ => Err(YmdError::InvalidNumber(ValidationError::MonthDoesNotExist)),
        }
    }

    fn validate_day(&self) -> Result<(), YmdError> {
        match (self.month, self.day, is_leap_year(self.year)) {
            (1 | 3 | 5 | 7 | 8 | 10 | 12, 1..=31, _)
            | (4 | 6 | 9 | 11, 1..=30, _)
            | (2, 1..=29, true)
            | (2, 1..=28, false) => Ok(()),
            _ => Err(YmdError::InvalidNumber(ValidationError::DayDoesNotExist(
                Month::from_u32(self.month).unwrap(),
                if self.month == 2 {
                    Some(self.year)
                } else {
                    None
                },
            ))),
        }
    }

    fn parse_num(&mut self, n: &str, ymd_ty: YmdType) -> Result<(), YmdError> {
        match n.parse::<u32>() {
            Ok(num) => match ymd_ty {
                YmdType::Year => self.year = num as i32,
                YmdType::Month => {
                    self.month = num;
                    self.validate_month()?;
                }
                YmdType::Day => {
                    self.day = num;
                    self.validate_day()?;
                }
            },
            _ => return Err(YmdError::NaN(ymd_ty, n.to_owned())),
        }
        Ok(())
    }

    fn handle_missing_year(&mut self) -> Result<(), YmdError> {
        let today = Local::today();
        let current_month = today.month();
        let current_day = today.day();
        self.year = today.year();

        match (
            self.month < current_month,
            self.month == current_month,
            self.day < current_day,
        ) {
            (true, _, _) | (false, true, true) => self.year += 1,
            _ => {}
        }

        if let (true, false, true) = (self.month == 2, is_leap_year(self.year), self.day == 29) {
            return Err(YmdError::InvalidNumber(ValidationError::DayDoesNotExist(
                Month::February,
                Some(self.year),
            )));
        }

        Ok(())
    }
}

impl From<Ymd> for NaiveDate {
    fn from(ymd: Ymd) -> NaiveDate {
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
    let message = if days_diff > Duration::zero() {
        "days left until"
    } else {
        "days has passed since"
    };
    println!(
        "{} {} {}, {}.",
        days_diff.num_days(),
        message,
        date,
        date.weekday()
    );
}
