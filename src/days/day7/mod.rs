use crate::days::day7::file_system::FileSystem;
use crate::days::day7::Command::{Cd, Ls};
use crate::days::Day;
use std::str::FromStr;

mod file_system;

pub struct Day7 {
    filesystem: FileSystem,
}

pub enum Command {
    Ls,
    Cd(String),
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_start_matches("$ ").split(" ").collect::<Vec<_>>();
        let command = s.get(0).ok_or("No command found".to_string())?;

        match *command {
            "cd" => {
                let name = *s.get(1).ok_or("No directory to cd into".to_string())?;
                Ok(Cd(name.into()))
            }
            "ls" => Ok(Ls),
            _ => unreachable!("This command has not been implemented yet"),
        }
    }
}

impl Day7 {
    pub fn new(input: &str) -> Result<Self, String> {
        let input = std::fs::read_to_string(input)
            .map_err(|e| e.to_string())?;

        Self::new_from_str(&input)
    }

    pub fn new_from_str(input: &str) -> Result<Self, String> {
        let mut filesystem = FileSystem::new();
        for line in input.lines() {
            if line.starts_with("$") {
                match line.parse::<Command>() {
                    Ok(Cd(name)) => filesystem.cd(&name).expect("Could not cd into dir"),
                    Ok(Ls) | _ => (),
                }

                continue;
            }

            let item_str = line.split_ascii_whitespace().collect::<Vec<_>>();
            match item_str.get(0) {
                Some(&"dir") => {
                    let dir_name = item_str.get(1).ok_or("No directory name".to_string())?;
                    filesystem.insert_dir(dir_name)
                }
                Some(s) if s.parse::<usize>().is_ok() => {
                    let size = s.parse::<usize>().unwrap();
                    let file_name = item_str.get(1).ok_or("No file name".to_string())?;
                    filesystem.insert_file(file_name, size);
                }
                _ => return Err("No valid file or directory found".into()),
            };
        }

        Ok(Day7 { filesystem })
    }
}

impl Day for Day7 {
    fn part1(&self) -> String {
        self.filesystem
            .find_all_matching_sizes(|item| item.size() <= 100000)
            .iter()
            .filter(|item| item.is_dir())
            .map(|item| item.size())
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self) -> String {
        let total_memory = 70000000;
        let desired_size = 30000000;
        let current_size = self.filesystem.size();
        let free_space = total_memory - current_size;
        let needed_space = desired_size - free_space;

        self.filesystem
            .find_all_matching_sizes(|item| item.size() >= needed_space)
            .iter()
            .filter(|item| item.is_dir())
            .map(|item| item.size())
            .min()
            .unwrap_or(0)
            .to_string()
    }

    fn day_number(&self) -> usize {
        7
    }
}

#[cfg(test)]
mod tests {
    use super::Day;
    use super::Day7;

    const INPUT: &'static str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    pub fn day7_part_1() {
        println!("testing");
        let result = Day7::new_from_str(INPUT).unwrap().part1();
        let expected = "95437";

        assert_eq!(result, expected);
    }

    #[test]
    pub fn day7_part_2() {
        let result = Day7::new_from_str(INPUT).unwrap().part2();
        let expected = "24933642";

        assert_eq!(result, expected);
    }
}
