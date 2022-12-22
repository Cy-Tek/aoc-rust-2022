mod map;
mod list;

use std::str::FromStr;

pub fn parse_lines<T: FromStr>(input: &str) -> Vec<T> {
    input.lines().flat_map(|line| line.parse()).collect()
}

// pub fn parse_from_n_lines<T: FromStr>(input: &str, num_lines: usize) -> Vec<T> {
//     let mut lines = input.lines().peekable();
//     let mut result = vec![];
//
//     while lines.peek().is_some() {
//         if let Ok(value) = lines.take(num_lines).clone().collect::<String>().parse() {
//             result.push(value);
//         }
//     }
//
//     result
// }
