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

struct Simulation<'a> {
    notes: &'a Notes,
    path_iter: slice::Iter<'a, Direction>,
    row: usize,
    column: usize,
    facing: usize,
}

impl<'a> Simulation<'a> {
    fn new(notes: &'a Notes) -> Self {
        Simulation {
            notes,
            path_iter: notes.path.iter(),
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
        }
    }

    fn new_coords(&self, old_column: usize, old_row: usize) -> (usize, usize) {
        let rows = self.notes.map.len();
        let columns = self.notes.map[old_row - 1].len();
        match self.facing {
            // right (column +)
            0 => ((old_column % columns) + 1, old_row),
            // down (row +)
            1 => (old_column, (old_row % rows) + 1),
            // left (column -)
            2 => (
                if old_column == 1 {
                    columns
                } else {
                    old_column - 1
                },
                old_row,
            ),
            // up (row -)
            3 => (old_column, if old_row == 1 { rows } else { old_row - 1 }),
            _ => (old_column, old_row),
        }
    }

    fn step_forward(&mut self) -> bool {
        let mut column = self.column;
        let mut row = self.row;
        loop {
            (column, row) = self.new_coords(column, row);
            match self
                .notes
                .map
                .get(row - 1)
                .and_then(|row| row.get(column - 1))
                .and_then(Option::as_ref)
            {
                Some(Tile::Wall) => {
                    return false;
                }
                Some(Tile::Open) => {
                    self.row = row;
                    self.column = column;
                    return true;
                }
                None => {}
            }
        }
    }
}

impl<'a> Iterator for Simulation<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self.path_iter.next()? {
            Direction::Left => {
                self.facing = (self.facing + 4 - 1) % 4;
            }
            Direction::Right => {
                self.facing = (self.facing + 1) % 4;
            }
            Direction::Forward(c) => {
                for _ in 0..*c {
                    if !self.step_forward() {
                        break;
                    }
                }
            }
        }

        Some(self.row * 1000 + self.column * 4 + self.facing)
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let notes = Notes::parse_nom(input).ok()?.1;
    let simulation = Simulation::new(&notes);
    simulation.last()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_one() {
        assert_eq!(part_one(include_str!("../example.txt")), Some(6032));
    }

    #[test]
    fn challenge_part_one() {
        assert_eq!(part_one(include_str!("../challenge.txt")), Some(149138));
    }
}
