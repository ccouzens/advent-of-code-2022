use std::{cmp::Ordering::*, collections::BTreeSet};

use nom::{
    branch::alt,
    character::complete::{char, digit1, line_ending},
    combinator::{iterator, map, map_res, value},
    sequence::{terminated, tuple},
    IResult,
};

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

use Direction::*;

#[derive(Debug)]
struct Motion {
    direction: Direction,
    steps: usize,
}

impl Iterator for Motion {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        self.steps = self.steps.checked_sub(1)?;
        Some(self.direction)
    }
}

fn parse_motion(input: &str) -> IResult<&str, Motion> {
    map(
        tuple((
            alt((
                value(Up, char('U')),
                value(Right, char('R')),
                value(Down, char('D')),
                value(Left, char('L')),
            )),
            char(' '),
            map_res(digit1, str::parse),
        )),
        |(direction, _, steps)| Motion { direction, steps },
    )(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
struct Position {
    x: i16,
    y: i16,
}

impl Position {
    fn move_direction(&mut self, direction: Direction) {
        match direction {
            Up => self.y += 1,
            Right => self.x += 1,
            Down => self.y -= 1,
            Left => self.x -= 1,
        }
    }

    fn is_touching(&mut self, other: &Self) -> bool {
        (-1..=1).contains(&(self.x - other.x)) && (-1..=1).contains(&(self.y - other.y))
    }

    fn move_towards(&mut self, other: &Self) {
        self.x += match self.x.cmp(&other.x) {
            Less => 1,
            Equal => 0,
            Greater => -1,
        };
        self.y += match self.y.cmp(&other.y) {
            Less => 1,
            Equal => 0,
            Greater => -1,
        }
    }
}

fn rope_simulation(input: &str, rope_length: usize) -> usize {
    let mut motions_iterator = iterator(input, terminated(parse_motion, line_ending));
    let mut rope = vec![Position::default(); rope_length];
    let mut visited = BTreeSet::new();
    for direction in &mut motions_iterator.flatten() {
        let mut previous_knot = None;
        for knot in rope.iter_mut() {
            match previous_knot {
                None => knot.move_direction(direction),
                Some(previous_knot) => {
                    if !knot.is_touching(previous_knot) {
                        knot.move_towards(previous_knot);
                    }
                }
            };
            previous_knot = Some(knot);
        }
        if let Some(last_knot) = previous_knot {
            visited.insert(*last_knot);
        }
    }
    visited.len()
}

pub fn part_one(input: &str) -> usize {
    rope_simulation(input, 2)
}

pub fn part_two(input: &str) -> usize {
    rope_simulation(input, 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_one() {
        assert_eq!(part_one(include_str!("../example1.txt")), 13);
    }

    #[test]
    fn challenge_part_one() {
        assert_eq!(part_one(include_str!("../challenge.txt")), 5735);
    }

    #[test]
    fn example_one_part_two() {
        assert_eq!(part_two(include_str!("../example1.txt")), 1);
    }

    #[test]
    fn example_two_part_two() {
        assert_eq!(part_two(include_str!("../example2.txt")), 36);
    }

    #[test]
    fn challenge_part_two() {
        assert_eq!(part_two(include_str!("../challenge.txt")), 2478);
    }
}
