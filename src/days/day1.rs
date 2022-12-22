use crate::days::Day;
use std::collections::VecDeque;
use std::path::Path;

pub struct Day1(String);

impl Day1 {
    pub fn new<P: AsRef<Path>>(file: P) -> Result<Self, String> {
        match std::fs::read_to_string(file) {
            Ok(s) => Ok(Self(s)),
            Err(e) => Err(e.to_string()),
        }
    }
}

impl Into<Box<dyn Day>> for Day1 {
    fn into(self) -> Box<dyn Day> {
        Box::new(self)
    }
}

impl Day for Day1 {
    fn part1(&self) -> String {
        self.0
            .split("\n\n")
            .map(|pack| {
                pack.split("\n")
                    .flat_map(|num| num.parse::<usize>())
                    .sum::<usize>()
            })
            .max()
            .map_or_else(|| "0".into(), |day| day.to_string())
    }

    fn part2(&self) -> String {
        let mut packs = self
            .0
            .split("\n\n")
            .map(|pack| {
                pack.split("\n")
                    .flat_map(|num| num.parse::<usize>())
                    .sum::<usize>()
            })
            .collect::<VecDeque<_>>();

        "".to_string()
    }

    fn day_number(&self) -> usize {
        1
    }
}
