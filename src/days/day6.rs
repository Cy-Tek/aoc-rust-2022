use crate::days::Day;
use std::collections::HashSet;
use std::path::Path;

pub struct Day6 {
    input: String,
}

impl Day6 {
    pub fn new<P: AsRef<Path>>(file: P) -> Result<Self, String> {
        std::fs::read_to_string(file)
            .map(|s| Self::from_string(&s))
            .map_err(|e| e.to_string())
    }

    fn from_string(input: &str) -> Self {
        Self {
            input: input.into(),
        }
    }

    fn find_marker(&self, marker_length: usize) -> String {
        let chars = self.input.chars().collect::<Vec<char>>();
        for (i, c_window) in chars.windows(marker_length).enumerate() {
            let set: HashSet<&char> = HashSet::from_iter(c_window.into_iter());
            if set.len() == marker_length {
                return (i + marker_length).to_string();
            }
        }

        0.to_string()
    }
}

impl Day for Day6 {
    fn part1(&self) -> String {
        self.find_marker(4)
    }

    fn part2(&self) -> String {
        self.find_marker(14)
    }

    fn day_number(&self) -> usize {
        6
    }
}

#[cfg(test)]
mod test {
    use crate::days::day6::Day6;
    use crate::days::Day;

    #[test]
    fn day6_part_1() {
        let inputs = vec![
            "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
            "bvwbjplbgvbhsrlpgdmjqwftvncz",
            "nppdvjthqldpwncqszvftbrmjlhg",
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
        ];

        let expected = vec![
            7.to_string(),
            5.to_string(),
            6.to_string(),
            10.to_string(),
            11.to_string(),
        ];

        for (input, expected) in inputs.iter().zip(expected) {
            let actual = Day6::from_string(input).part1();
            assert_eq!(actual, expected)
        }
    }

    #[test]
    fn day6_part_2() {
        let inputs = vec![
            "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
            "bvwbjplbgvbhsrlpgdmjqwftvncz",
            "nppdvjthqldpwncqszvftbrmjlhg",
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
        ];

        let expected = vec![
            19.to_string(),
            23.to_string(),
            23.to_string(),
            29.to_string(),
            26.to_string(),
        ];

        for (input, expected) in inputs.iter().zip(expected) {
            let actual = Day6::from_string(input).part2();
            assert_eq!(actual, expected)
        }
    }
}
