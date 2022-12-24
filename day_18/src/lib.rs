use std::collections::BTreeSet;

use nom::{
    character::complete::{char, digit1, newline},
    combinator::{iterator, map_res},
    sequence::{terminated, tuple},
    IResult,
};

#[derive(Debug)]
struct RockScan(BTreeSet<(i8, i8, i8)>);

impl RockScan {
    fn new(input: &str) -> Self {
        fn parse_rock(input: &str) -> IResult<&str, (i8, char, i8, char, i8)> {
            tuple((
                map_res(digit1, str::parse),
                char(','),
                map_res(digit1, str::parse),
                char(','),
                map_res(digit1, str::parse),
            ))(input)
        }
        let mut rocks = BTreeSet::new();
        for (x, _, y, _, z) in &mut iterator(input, terminated(parse_rock, newline)) {
            rocks.insert((x, y, z));
        }
        Self(rocks)
    }
}

pub fn part_one(input: &str) -> usize {
    let rock_scan = RockScan::new(input).0;
    rock_scan
        .iter()
        .map(|&(x, y, z)| {
            6 - usize::from(rock_scan.contains(&(x - 1, y, z)))
                - usize::from(rock_scan.contains(&(x + 1, y, z)))
                - usize::from(rock_scan.contains(&(x, y - 1, z)))
                - usize::from(rock_scan.contains(&(x, y + 1, z)))
                - usize::from(rock_scan.contains(&(x, y, z - 1)))
                - usize::from(rock_scan.contains(&(x, y, z + 1)))
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_one() {
        assert_eq!(part_one(include_str!("../example.txt")), 64);
    }

    #[test]
    fn challenge_part_one() {
        assert_eq!(part_one(include_str!("../challenge.txt")), 3650);
    }
}
