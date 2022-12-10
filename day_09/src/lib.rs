use std::{cmp::Ordering::*, collections::BTreeSet, iter::repeat};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    x: usize,
    y: usize,
}

impl Default for Position {
    fn default() -> Self {
        Self {
            x: usize::MAX / 2,
            y: usize::MAX / 2,
        }
    }
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
        (self.x == other.x || self.x + 1 == other.x || self.x == other.x + 1)
            && (self.y == other.y || self.y + 1 == other.y || self.y == other.y + 1)
    }

    fn move_towards(&mut self, other: &Self) {
        match self.x.cmp(&other.x) {
            Less => self.x += 1,
            Equal => {}
            Greater => self.x -= 1,
        }
        match self.y.cmp(&other.y) {
            Less => self.y += 1,
            Equal => {}
            Greater => self.y -= 1,
        }
    }
}

impl Motion {
    fn iter(&self) -> impl Iterator<Item = Direction> {
        repeat(self.direction).take(self.steps)
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

pub fn part_one(input: &str) -> usize {
    let mut motions_iterator = iterator(input, terminated(parse_motion, line_ending));
    let mut head = Position::default();
    let mut tail = head;
    let mut visited = BTreeSet::new();
    visited.insert(tail);
    for direction in &mut motions_iterator.flat_map(|m| m.iter()) {
        head.move_direction(direction);
        if !tail.is_touching(&head) {
            tail.move_towards(&head);
        }
        visited.insert(tail);
    }
    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_one() {
        assert_eq!(part_one(include_str!("../example.txt")), 13);
    }

    #[test]
    fn challenge_part_one() {
        assert_eq!(part_one(include_str!("../challenge.txt")), 5735);
    }
}
