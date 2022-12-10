use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, line_ending, not_line_ending},
    combinator::{iterator, map, map_res},
    sequence::{preceded, terminated, tuple},
    IResult,
};

#[derive(Debug)]
enum ConsoleLine<'a> {
    Cd(&'a str),
    Ls,
    Directory(&'a str),
    File { size: u64 },
}

fn parse_command(input: &str) -> IResult<&str, ConsoleLine> {
    alt((
        preceded(tag("$ cd "), map(not_line_ending, ConsoleLine::Cd)),
        map(tag("$ ls"), |_| ConsoleLine::Ls),
        preceded(tag("dir "), map(not_line_ending, ConsoleLine::Directory)),
        map(
            tuple((map_res(digit1, str::parse), char(' '), not_line_ending)),
            |(size, _, _name)| ConsoleLine::File { size },
        ),
    ))(input)
}

#[derive(Debug, Default)]
struct FSTreeDirectory<'a> {
    children: HashMap<&'a str, usize>,
    size: u64,
}

struct FileSystem<'a> {
    nodes: Vec<FSTreeDirectory<'a>>,
}

impl<'a> FileSystem<'a> {
    fn dir_at_index(&mut self, index: usize) -> Result<&mut FSTreeDirectory<'a>, &'static str> {
        self.nodes
            .get_mut(index)
            .ok_or("Failed to find node in filesystem")
    }

    fn new_from_observations(input: &'a str) -> Result<Self, &'static str> {
        let mut command_iter = iterator(input, terminated(parse_command, line_ending));
        let mut filesystem = FileSystem {
            nodes: vec![FSTreeDirectory::default()],
        };
        let mut stack = vec![0];
        for command in &mut command_iter {
            let dir_count = filesystem.nodes.len();
            let current_index = *stack.last().ok_or("Expected stack of directories")?;
            let current_directory = filesystem.dir_at_index(current_index)?;

            match command {
                ConsoleLine::Cd("/") => {
                    stack = vec![0];
                }
                ConsoleLine::Ls => {}
                ConsoleLine::Cd("..") => {
                    stack.pop();
                }
                ConsoleLine::Cd(name) => {
                    stack.push(
                        *current_directory
                            .children
                            .get(name)
                            .ok_or("Failed to find directory by name")?,
                    );
                }
                ConsoleLine::Directory(name) => {
                    current_directory.children.insert(name, dir_count);
                    filesystem.nodes.push(FSTreeDirectory::default());
                }
                ConsoleLine::File { size } => {
                    for index in stack.iter() {
                        let dir = filesystem.dir_at_index(*index)?;
                        dir.size += size;
                    }
                }
            }
        }
        Ok(filesystem)
    }

    fn traverse_dir_sizes(&self) -> impl Iterator<Item = u64> + '_ {
        self.nodes.iter().map(|n| n.size)
    }
}

pub fn part_one(input: &str) -> Result<u64, &'static str> {
    let tree = FileSystem::new_from_observations(input)?;
    Ok(tree
        .traverse_dir_sizes()
        .filter(|&size| size <= 100000)
        .sum())
}

pub fn part_two(input: &str) -> Result<u64, &'static str> {
    let tree = FileSystem::new_from_observations(input)?;
    let root = tree.nodes.first().ok_or("Failed to find root node")?;
    let space_needed = 30000000 - (70000000 - root.size);

    tree.traverse_dir_sizes()
        .filter(|&size| size >= space_needed)
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
