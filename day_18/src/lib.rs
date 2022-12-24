use std::{collections::BTreeSet, mem::take};

use nom::{
    character::complete::{char, digit1, newline},
    combinator::{iterator, map_res},
    sequence::{terminated, tuple},
    IResult,
};

#[derive(Debug, Default)]
struct Scan(BTreeSet<(i8, i8, i8)>);

impl Scan {
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

    fn surface_area(&self) -> usize {
        self.0
            .iter()
            .map(|&(x, y, z)| {
                6 - usize::from(self.0.contains(&(x - 1, y, z)))
                    - usize::from(self.0.contains(&(x + 1, y, z)))
                    - usize::from(self.0.contains(&(x, y - 1, z)))
                    - usize::from(self.0.contains(&(x, y + 1, z)))
                    - usize::from(self.0.contains(&(x, y, z - 1)))
                    - usize::from(self.0.contains(&(x, y, z + 1)))
            })
            .sum()
    }
}

pub fn part_one(input: &str) -> usize {
    Scan::new(input).surface_area()
}

pub fn part_two(input: &str) -> usize {
    let rock_scan = Scan::new(input).0;
    let x_min = rock_scan
        .iter()
        .map(|&(x, _, _)| x)
        .min()
        .unwrap_or_default()
        - 1;
    let x_max = rock_scan
        .iter()
        .map(|&(x, _, _)| x)
        .max()
        .unwrap_or_default()
        + 1;
    let y_min = rock_scan
        .iter()
        .map(|&(_, y, _)| y)
        .min()
        .unwrap_or_default()
        - 1;
    let y_max = rock_scan
        .iter()
        .map(|&(_, y, _)| y)
        .max()
        .unwrap_or_default()
        + 1;
    let z_min = rock_scan
        .iter()
        .map(|&(_, _, z)| z)
        .min()
        .unwrap_or_default()
        - 1;
    let z_max = rock_scan
        .iter()
        .map(|&(_, _, z)| z)
        .max()
        .unwrap_or_default()
        + 1;
    let mut external_scan = Scan::default();
    external_scan.0.insert((x_min, y_min, z_min));
    let mut recent = vec![(x_min, y_min, z_min)];
    let mut visit = |(x, y, z): (i8, i8, i8), recent: &mut Vec<(i8, i8, i8)>| {
        if !rock_scan.contains(&(x, y, z)) && external_scan.0.insert((x, y, z)) {
            recent.push((x, y, z));
        }
    };

    while !recent.is_empty() {
        for &(x, y, z) in take(&mut recent).iter() {
            if x > x_min {
                visit((x - 1, y, z), &mut recent);
            }
            if x < x_max {
                visit((x + 1, y, z), &mut recent);
            }
            if y > y_min {
                visit((x, y - 1, z), &mut recent);
            }
            if y < y_max {
                visit((x, y + 1, z), &mut recent);
            }
            if z > z_min {
                visit((x, y, z - 1), &mut recent);
            }
            if z < z_max {
                visit((x, y, z + 1), &mut recent);
            }
        }
    }

    let width = (x_max + 1 - x_min) as usize;
    let height = (y_max + 1 - y_min) as usize;
    let depth = (z_max + 1 - z_min) as usize;

    external_scan.surface_area() - width * height * 2 - width * depth * 2 - height * depth * 2
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

    #[test]
    fn example_part_two() {
        assert_eq!(part_two(include_str!("../example.txt")), 58);
    }

    #[test]
    fn challenge_part_two() {
        assert_eq!(part_two(include_str!("../challenge.txt")), 2118);
    }
}
