#![feature(is_some_and)]

use crate::days::day1::Day1;
use crate::days::day5::Day5;
use crate::days::day6::Day6;
use crate::days::Day;
use crate::days::day7::Day7;

mod days;
mod utils;

fn main() {
    let days = match make_days() {
        Ok(days) => days,
        Err(e) => panic!("{}", e.to_string()),
    };

    for day in days {
        println!("{}\n", day);
    }
}

fn make_days() -> Result<Vec<Box<dyn Day>>, String> {
    Ok(vec![
        Box::new(Day1::new("inputs/day1.txt")?),
        Box::new(Day5::new("inputs/day5.txt")?),
        Box::new(Day6::new("inputs/day6.txt")?),
        Box::new(Day7::new("inputs/day7.txt")?),
    ])
}
