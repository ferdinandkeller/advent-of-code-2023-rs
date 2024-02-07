mod day1;

use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1) {
        Some(day) => match day.as_str() {
            "day1a" => day1::solve_a(read_to_string("inputs/day1.txt")?.as_str()),
            "day1b" => day1::solve_b(read_to_string("inputs/day1.txt")?.as_str()),
            _ => Err("Invalid day!".into()),
        },
        None => Err("Please specify a day!".into()),
    }
}
