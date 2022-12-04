#![no_std]

use nom::{
    character::complete::{char, digit1},
    combinator::{map, map_res},
    sequence::separated_pair,
    IResult,
};

#[derive(Debug)]
struct SectionAssignment(u64, u64);

impl SectionAssignment {
    fn fully_contains(&self, other: &Self) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.1 >= other.0 && other.1 >= self.0
    }
}

fn parse_section_assignments(input: &str) -> IResult<&str, SectionAssignment> {
    map(
        separated_pair(
            map_res(digit1, str::parse),
            char('-'),
            map_res(digit1, str::parse),
        ),
        |p| SectionAssignment(p.0, p.1),
    )(input)
}

fn parse_elf_pair(input: &str) -> IResult<&str, (SectionAssignment, SectionAssignment)> {
    separated_pair(
        parse_section_assignments,
        char(','),
        parse_section_assignments,
    )(input)
}

fn elf_pairs(input: &str) -> impl Iterator<Item = (SectionAssignment, SectionAssignment)> + '_ {
    input
        .lines()
        .filter_map(|line| parse_elf_pair(line).ok().map(|(_, pair)| pair))
}

pub fn part_one(input: &str) -> usize {
    elf_pairs(input)
        .filter(|(a, b)| a.fully_contains(b) || b.fully_contains(a))
        .count()
}

pub fn part_two(input: &str) -> usize {
    elf_pairs(input).filter(|(a, b)| a.overlaps(b)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_one() {
        assert_eq!(part_one(include_str!("../example.txt")), 2);
    }

    #[test]
    fn challenge_part_one() {
        assert_eq!(part_one(include_str!("../challenge.txt")), 584);
    }

    #[test]
    fn example_part_two() {
        assert_eq!(part_two(include_str!("../example.txt")), 4);
    }

    #[test]
    fn challenge_part_two() {
        assert_eq!(part_two(include_str!("../challenge.txt")), 933);
    }
}
