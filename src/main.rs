use chrono::{Date, Datelike, Duration, Local, NaiveDate};

fn main() {
    let mut args = std::env::args().skip(1);

    if args.len() != 1 {
        eprintln!(
            "Wrong number of inputs; expected 1, but given {}.\nUSAGE: cargo run mm/dd/yyyy // you can alternatively omit year",
            args.len()
        );
        std::process::exit(1);
    }

    if let Some(date_string) = args.next() {
        // mdy: [month, day, year]
        let mdy = date_string
            .split('/')
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let today = Date::naive_local(&Local::today());
        let is_leap_year = if mdy.len() == 3 {
            is_leap_year(mdy[2])
        } else {
            is_leap_year(today.year() as u32)
        };

        let year = if mdy.len() == 3 {
            mdy[2] as i32
        } else {
            today.year()
        };

        (match (mdy[0], mdy[1], is_leap_year) {
            (1 | 3 | 5 | 7 | 8 | 10 | 12, 1..=31, _) => Ok(()),
            (4 | 6 | 9 | 11, 1..=30, _) => Ok(()),
            (2, 1..=29, true) => Ok(()),
            (2, 1..=28, false) => Ok(()),
            _ => Err("The given input could not get validated."),
        })
        .expect("All month, day, and year inputs are expected to be correct - meaning that it cannot be something like 9/31/2022, which does not exist and will lead to failure.");

        let date_input = NaiveDate::from_ymd(year, mdy[0], mdy[1]);
        let days_diff = date_input - today;

        if days_diff > Duration::zero() {
            println!(
                "{} days left until {}, {}.",
                days_diff.num_days(),
                date_input,
                date_input.weekday()
            );
        } else {
            println!(
                "{} days has passed since {}, {}.",
                -days_diff.num_days(),
                date_input,
                date_input.weekday()
            );
        }
    }
}

fn is_leap_year(year: u32) -> bool {
    year % 400 == 0 || (year % 4 == 0 && year % 100 != 0)
}
