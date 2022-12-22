pub mod day1;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;

use std::fmt::{Display, Formatter};

pub trait Day {
    fn part1(&self) -> String;
    fn part2(&self) -> String;
    fn day_number(&self) -> usize;
}

impl Display for dyn Day {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Day {}:\n\
        -------------------------\n\
        Part 1: {}\n\
        Part 2: {}",
            self.day_number(),
            self.part1(),
            self.part2()
        )
    }
}
