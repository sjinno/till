mod ymd;

use crate::ymd::{count_days, print_days_diff, Ymd};

fn main() {
    let mut args = std::env::args().skip(1);

    if args.len() != 1 {
        eprintln!(
            "Wrong number of inputs; expected 1, but given {}.\nUSAGE: cargo run mm/dd/yyyy // you can optionally omit year input",
            args.len()
        );
        std::process::exit(1);
    }

    let date = Ymd::new(args.next().unwrap()).into();
    let days_diff = count_days(date);

    print_days_diff(days_diff, date);
}
