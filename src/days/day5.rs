use crate::days::Day;
use crate::utils;
use std::num::ParseIntError;
use std::path::Path;
use std::str::FromStr;

#[derive(Copy, Clone)]
struct Instruction {
    number: usize,
    from_idx: usize,
    to_idx: usize,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums = s.split_ascii_whitespace().collect::<Vec<_>>();
        let number = nums[1]
            .parse::<usize>()
            .map_err(|e: ParseIntError| e.to_string())?;
        let from_idx = nums[3]
            .parse::<usize>()
            .map_err(|e: ParseIntError| e.to_string())?
            - 1;
        let to_idx = nums[5]
            .parse::<usize>()
            .map_err(|e: ParseIntError| e.to_string())?
            - 1;

        Ok(Self {
            number,
            from_idx,
            to_idx,
        })
    }
}

pub struct Day5 {
    stacks: Vec<Vec<char>>,
    instructions: Vec<Instruction>,
}

impl Day5 {
    pub fn new<P: AsRef<Path>>(file: P) -> Result<Self, String> {
        let input = std::fs::read_to_string(file).map_err(|e| e.to_string())?;
        Self::from_string(&input)
    }

    fn from_string(input: &str) -> Result<Self, String> {
        let mut lines = input.lines().peekable();
        let mut stack_str = String::new();

        while let Some(line) = lines.next_if(|s| !s.chars().nth(1).is_some_and(char::is_numeric)) {
            stack_str += line;
            stack_str += "\n";
        }

        let columns = lines
            .next()
            .map(|s| s.split_ascii_whitespace().count())
            .ok_or("No lines to create columns from")?;

        let mut stacks = vec![Vec::new(); columns];
        Self::fill_stacks(&mut stacks, &stack_str);

        let instructions = utils::parse_lines(&lines.skip(1).collect::<Vec<_>>().join("\n"));

        Ok(Self {
            stacks,
            instructions,
        })
    }

    fn move_stack(instruction: Instruction, stacks: &mut [Vec<char>]) {
        let Instruction {
            number,
            to_idx,
            from_idx,
        } = instruction;

        for _ in 0..number {
            if let Some(item) = stacks[from_idx].pop() {
                stacks[to_idx].push(item);
            }
        }
    }

    fn copy_stack(instruction: Instruction, stacks: &mut [Vec<char>]) {
        let Instruction {
            number,
            to_idx,
            from_idx,
        } = instruction;

        let mut from_stack = stacks[from_idx]
            .drain(stacks[from_idx].len() - number..)
            .collect();

        stacks[to_idx].append(&mut from_stack);
    }

    fn fill_stacks(stacks: &mut Vec<Vec<char>>, input: &str) {
        for line in input.lines().rev() {
            Self::fill_stack(stacks, line);
        }
    }

    fn fill_stack(stacks: &mut [Vec<char>], line: &str) {
        let mut col = 0;
        for ch in line.chars().skip(1).step_by(4) {
            if !ch.is_alphabetic() {
                col += 1;
                continue;
            }

            if let Some(stack) = stacks.get_mut(col) {
                stack.push(ch);
            }

            col += 1;
        }
    }
}

impl Day for Day5 {
    fn part1(&self) -> String {
        let mut stacks = self.stacks.clone();

        for instruction in &self.instructions {
            Day5::move_stack(instruction.clone(), &mut stacks)
        }

        let mut result = String::new();
        for stack in stacks {
            if let Some(ch) = stack.last() {
                result.push(*ch);
            }
        }

        result
    }

    fn part2(&self) -> String {
        let mut stacks = self.stacks.clone();

        for instruction in &self.instructions {
            Day5::copy_stack(instruction.clone(), &mut stacks)
        }

        let mut result = String::new();
        for stack in stacks {
            if let Some(ch) = stack.last() {
                result.push(*ch);
            }
        }

        result
    }

    fn day_number(&self) -> usize {
        5
    }
}

#[cfg(test)]
mod test {
    use crate::days::Day;

    #[test]
    fn day5_part_1() {
        let input = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        let actual = super::Day5::from_string(&input).unwrap().part1();
        let expected = "CMZ";

        assert_eq!(actual, expected);
    }

    #[test]
    fn day5_part_2() {
        let input = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        let actual = super::Day5::from_string(&input).unwrap().part2();
        let expected = "MCD";

        assert_eq!(actual, expected);
    }
}
