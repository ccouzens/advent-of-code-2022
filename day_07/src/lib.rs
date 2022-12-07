use std::{
    collections::{hash_map::Entry, HashMap},
    iter::once,
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, newline, not_line_ending},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Debug, PartialEq)]
enum ConsoleLine<'a> {
    Cd(&'a str),
    Ls,
    Directory(&'a str),
    File { size: u64 },
}

fn parse_cd(input: &str) -> IResult<&str, ConsoleLine> {
    preceded(tag("$ cd "), map(not_line_ending, ConsoleLine::Cd))(input)
}

fn parse_ls(input: &str) -> IResult<&str, ConsoleLine> {
    map(tag("$ ls"), |_| ConsoleLine::Ls)(input)
}

fn parse_directory(input: &str) -> IResult<&str, ConsoleLine> {
    preceded(tag("dir "), map(not_line_ending, ConsoleLine::Directory))(input)
}

fn parse_file(input: &str) -> IResult<&str, ConsoleLine> {
    map(
        tuple((map_res(digit1, str::parse), char(' '), not_line_ending)),
        |(size, _, _name)| ConsoleLine::File { size },
    )(input)
}

fn parse_commands(input: &str) -> IResult<&str, Vec<ConsoleLine>> {
    separated_list1(
        newline,
        alt((parse_cd, parse_ls, parse_directory, parse_file)),
    )(input)
}

#[derive(Debug, Default)]
struct FSTreeDirectory<'a> {
    children: HashMap<&'a str, FSTreeDirectory<'a>>,
    size: u64,
}

struct FSTreeDirectoryIterator<'a, 'b> {
    stack: Vec<std::collections::hash_map::Values<'a, &'b str, FSTreeDirectory<'b>>>,
}

impl<'a, 'b> Iterator for FSTreeDirectoryIterator<'a, 'b> {
    type Item = &'a FSTreeDirectory<'b>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut bottom_of_stack = self.stack.pop()?;
            if let Some(dir) = bottom_of_stack.next() {
                self.stack.push(bottom_of_stack);
                self.stack.push(dir.children.values());
                return Some(dir);
            }
        }
    }
}

impl<'a> FSTreeDirectory<'a> {
    fn new_from_observations(input: &'a str) -> Result<FSTreeDirectory<'a>, &'static str> {
        let commands = parse_commands(input).map_err(|_| "Error parsing input")?.1;
        let mut root = FSTreeDirectory::default();
        let mut stack = vec![];
        for (i, command) in commands.iter().enumerate() {
            match (i, command) {
                (0, ConsoleLine::Cd("/")) => {}
                (0, _) => return Err("Expected to start at /"),
                (_, ConsoleLine::Ls) => {}
                (_, ConsoleLine::Cd("..")) => {
                    stack.pop();
                }
                (_, ConsoleLine::Cd(name)) => {
                    stack.push(name);
                }
                (_, ConsoleLine::Directory(name)) => {
                    let mut current_dir = &mut root;
                    for name in stack.iter() {
                        match current_dir.children.get_mut(**name) {
                            Some(dir) => current_dir = dir,
                            None => return Err("Missing directory in stack"),
                        }
                    }
                    match current_dir.children.entry(name) {
                        Entry::Occupied(_) => {
                            return Err("Directory already taken");
                        }
                        Entry::Vacant(v) => {
                            v.insert(FSTreeDirectory::default());
                        }
                    }
                }
                (_, ConsoleLine::File { size }) => {
                    root.size += size;
                    let mut current_dir = &mut root;
                    for name in stack.iter() {
                        match current_dir.children.get_mut(**name) {
                            Some(dir) => {
                                dir.size += size;
                                current_dir = dir
                            }
                            None => return Err("Missing directory in stack"),
                        }
                    }
                }
            }
        }
        Ok(root)
    }

    fn traverse_nodes(&self) -> impl Iterator<Item = &FSTreeDirectory<'a>> {
        FSTreeDirectoryIterator {
            stack: vec![self.children.values()],
        }
        .chain(once(self))
    }
}

pub fn part_one(input: &str) -> Result<u64, &'static str> {
    let tree = FSTreeDirectory::new_from_observations(input)?;
    Ok(tree
        .traverse_nodes()
        .map(|dir| dir.size)
        .filter(|&size| size <= 100000)
        .sum())
}

pub fn part_two(input: &str) -> Result<u64, &'static str> {
    let tree = FSTreeDirectory::new_from_observations(input)?;
    let space_needed = 30000000 - (70000000 - tree.size);

    dbg!(space_needed);
    tree.traverse_nodes()
        .flat_map(|dir| {
            if dir.size >= space_needed {
                Some(dir.size)
            } else {
                None
            }
        })
        .min()
        .ok_or("Failed to find big enough directory")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_one() {
        assert_eq!(part_one(include_str!("../example.txt")), Ok(95437));
    }

    #[test]
    fn challenge_part_one() {
        assert_eq!(part_one(include_str!("../challenge.txt")), Ok(1367870));
    }

    #[test]
    fn example_part_two() {
        assert_eq!(part_two(include_str!("../example.txt")), Ok(24933642));
    }

    #[test]
    fn challenge_part_two() {
        assert_eq!(part_two(include_str!("../challenge.txt")), Ok(549173));
    }
}
