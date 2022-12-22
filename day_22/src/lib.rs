use std::slice;

use nom::{
    branch::alt,
    character::complete::{char, digit1, newline},
    combinator::{map, map_res, value},
    multi::{many1, separated_list1},
    sequence::tuple,
    IResult,
};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
    Open,
    Wall,
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Left,
    Right,
    Forward(usize),
}

#[derive(Debug)]
struct Notes {
    map: Vec<Vec<Option<Tile>>>,
    path: Vec<Direction>,
}

impl Notes {
    fn parse_nom(input: &str) -> IResult<&str, Notes> {
        map(
            tuple((
                separated_list1(
                    newline,
                    many1(alt((
                        value(None, char(' ')),
                        value(Some(Tile::Open), char('.')),
                        value(Some(Tile::Wall), char('#')),
                    ))),
                ),
                newline,
                newline,
                many1(alt((
                    value(Direction::Left, char('L')),
                    value(Direction::Right, char('R')),
                    map_res(digit1, |d: &str| d.parse().map(Direction::Forward)),
                ))),
            )),
            |(map, _, _, path)| Self { map, path },
        )(input)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Position {
    row: usize,
    column: usize,
    facing: usize,
}

struct Simulation<'a> {
    notes: &'a Notes,
    path_iter: slice::Iter<'a, Direction>,
    position: Position,
    warping_rules: &'a fn(Position) -> Option<Position>,
}

impl<'a> Simulation<'a> {
    fn new(notes: &'a Notes, warping_rules: &'a fn(Position) -> Option<Position>) -> Self {
        Simulation {
            notes,
            path_iter: notes.path.iter(),
            position: Position {
                row: 1,
                column: notes
                    .map
                    .get(0)
                    .unwrap()
                    .iter()
                    .position(|&t| t == Some(Tile::Open))
                    .unwrap()
                    + 1,
                facing: 0,
            },
            warping_rules,
        }
    }

    fn new_coords(&self, old: Position) -> Position {
        (self.warping_rules)(old).unwrap_or(match old.facing {
            // right (column +)
            0 => Position {
                column: old.column + 1,
                ..old
            },
            // down (row +)
            1 => Position {
                row: old.row + 1,
                ..old
            },
            // left (column -)
            2 => Position {
                column: old.column - 1,
                ..old
            },
            // up (row -)
            3 => Position {
                row: old.row - 1,
                ..old
            },
            _ => old,
        })
    }

    fn step_forward(&mut self) -> bool {
        let position = self.new_coords(self.position);
        match self
            .notes
            .map
            .get(position.row - 1)
            .and_then(|row| row.get(position.column - 1))
            .and_then(Option::as_ref)
            .unwrap()
        {
            Tile::Wall => false,
            Tile::Open => {
                self.position = position;
                true
            }
        }
    }
}

impl<'a> Iterator for Simulation<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self.path_iter.next()? {
            Direction::Left => {
                self.position.facing = (self.position.facing + 4 - 1) % 4;
            }
            Direction::Right => {
                self.position.facing = (self.position.facing + 1) % 4;
            }
            Direction::Forward(c) => {
                for _ in 0..*c {
                    if !self.step_forward() {
                        break;
                    }
                }
            }
        }

        Some(self.position.row * 1000 + self.position.column * 4 + self.position.facing)
    }
}

pub fn secret(input: &str, warping_rules: fn(Position) -> Option<Position>) -> Option<usize> {
    let notes = Notes::parse_nom(input).ok()?.1;
    let simulation = Simulation::new(&notes, &warping_rules);
    simulation.last()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_one() {
        assert_eq!(
            secret(include_str!("../example.txt"), |p| match p {
                Position {
                    row: 1,
                    column: 9..=12,
                    facing: 3,
                } => Some(Position { row: 12, ..p }),
                Position {
                    row: 1..=4,
                    column: 8,
                    facing: 2,
                } => Some(Position { column: 12, ..p }),
                Position {
                    row: 1..=4,
                    column: 12,
                    facing: 0,
                } => Some(Position { column: 8, ..p }),
                Position {
                    row: 5,
                    column: 0..=8,
                    facing: 3,
                } => Some(Position { row: 8, ..p }),
                Position {
                    row: 5..=8,
                    column: 1,
                    facing: 2,
                } => Some(Position { column: 12, ..p }),
                Position {
                    row: 5..=8,
                    column: 12,
                    facing: 0,
                } => Some(Position { column: 1, ..p }),
                Position {
                    row: 8,
                    column: 1..=8,
                    facing: 1,
                } => Some(Position { row: 5, ..p }),

                _ => None,
            }),
            Some(6032)
        );
    }

    #[test]
    fn challenge_part_one() {
        assert_eq!(
            secret(include_str!("../challenge.txt"), |p| match p {
                Position {
                    row: 1,
                    column: 51..=100,
                    facing: 3,
                } => Some(Position { row: 150, ..p }),
                Position {
                    row: 1,
                    column: 101..=150,
                    facing: 3,
                } => Some(Position { row: 50, ..p }),
                Position {
                    row: 51..=100,
                    column: 51,
                    facing: 2,
                } => Some(Position { column: 100, ..p }),
                Position {
                    row: 1..=50,
                    column: 51,
                    facing: 2,
                } => Some(Position { column: 150, ..p }),
                Position {
                    row: 1..=50,
                    column: 150,
                    facing: 0,
                } => Some(Position { column: 51, ..p }),
                Position {
                    row: 50,
                    column: 101..=150,
                    facing: 1,
                } => Some(Position { row: 1, ..p }),
                Position {
                    row: 51..=100,
                    column: 100,
                    facing: 0,
                } => Some(Position { column: 51, ..p }),
                Position {
                    row: 101,
                    column: 1..=50,
                    facing: 3,
                } => Some(Position { row: 200, ..p }),
                Position {
                    row: 101..=150,
                    column: 1,
                    facing: 2,
                } => Some(Position { column: 100, ..p }),
                Position {
                    row: 101..=150,
                    column: 100,
                    facing: 0,
                } => Some(Position { column: 1, ..p }),
                Position {
                    row: 150,
                    column: 51..=100,
                    facing: 1,
                } => Some(Position { row: 1, ..p }),
                Position {
                    row: 151..=200,
                    column: 1,
                    facing: 2,
                } => Some(Position { column: 50, ..p }),
                Position {
                    row: 151..=200,
                    column: 50,
                    facing: 0,
                } => Some(Position { column: 1, ..p }),
                Position {
                    row: 200,
                    column: 1..=50,
                    facing: 1,
                } => Some(Position { row: 101, ..p }),

                _ => None,
            }),
            Some(149138)
        );
    }

    #[test]
    fn example_part_two() {
        assert_eq!(
            secret(include_str!("../example.txt"), |p| match p {
                Position {
                    row: 1,
                    column: 9..=12,
                    facing: 3,
                } => Some(Position {
                    row: 5,
                    facing: 1,
                    column: 12 - p.column + 1
                }),
                Position {
                    row: 1..=4,
                    column: 9,
                    facing: 2,
                } => Some(Position {
                    row: 5,
                    facing: 1,
                    column: p.row + 4
                }),
                Position {
                    row: 1..=4,
                    column: 12,
                    facing: 0,
                } => Some(Position {
                    facing: 2,
                    column: 16,
                    row: 4 - p.row + 8 + 1
                }),
                Position {
                    row: 5,
                    column: 1..=4,
                    facing: 3,
                } => Some(Position {
                    facing: 1,
                    row: 1,
                    column: 4 - p.column + 8 + 1
                }),
                Position {
                    row: 5,
                    column: 5..=8,
                    facing: 3,
                } => Some(Position {
                    facing: 0,
                    column: 9,
                    row: p.column - 4
                }),
                Position {
                    row: 5..=8,
                    column: 1,
                    facing: 2,
                } => Some(Position {
                    facing: 3,
                    row: 12,
                    column: 8 - p.row + 13
                }),
                Position {
                    row: 5..=8,
                    column: 12,
                    facing: 0,
                } => Some(Position {
                    facing: 1,
                    row: 9,
                    column: 8 - p.row + 13
                }),
                Position {
                    row: 8,
                    column: 1..=4,
                    facing: 1,
                } => Some(Position {
                    facing: 3,
                    row: 12,
                    column: 4 - p.column + 9
                }),
                Position {
                    row: 8,
                    column: 5..=9,
                    facing: 1,
                } => Some(Position {
                    facing: 2,
                    column: 9,
                    row: 8 - p.column + 9,
                }),
                Position {
                    row: 9,
                    column: 13..=16,
                    facing: 3,
                } => Some(Position {
                    facing: 0,
                    column: 12,
                    row: 16 - p.column + 5,
                }),
                Position {
                    row: 9..=12,
                    column: 9,
                    facing: 2,
                } => Some(Position {
                    row: 8,
                    column: 12 - p.row + 5,
                    facing: 3
                }),
                Position {
                    row: 9..=12,
                    column: 16,
                    facing: 0,
                } => Some(Position {
                    row: 12 - p.row + 1,
                    column: 12,
                    facing: 2
                }),
                Position {
                    row: 12,
                    column: 9..=12,
                    facing: 1,
                } => Some(Position {
                    row: 8,
                    column: 12 - p.column + 1,
                    facing: 3
                }),
                Position {
                    row: 12,
                    column: 13..=16,
                    facing: 1,
                } => Some(Position {
                    row: 16 - p.column + 5,
                    column: 1,
                    facing: 0
                }),
                _ => None,
            }),
            Some(5031)
        );
    }
}
