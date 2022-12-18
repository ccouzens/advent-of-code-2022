use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, newline},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::{cmp::Ordering::*, collections::BTreeSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Coordinate {
    x: u16,
    y: u16,
}

impl Coordinate {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            separated_pair(
                map_res(digit1, str::parse),
                char(','),
                map_res(digit1, str::parse),
            ),
            |(x, y)| Self { x, y },
        )(input)
    }

    fn iter_to_other(self, other: Self) -> impl Iterator<Item = Self> {
        (0..).map_while(
            move |i| match (self.x.cmp(&other.x), self.y.cmp(&other.y)) {
                (Less, Less) => None,
                (Less, Equal) => {
                    if self.x + i <= other.x {
                        Some(Self {
                            x: self.x + i,
                            ..self
                        })
                    } else {
                        None
                    }
                }
                (Less, Greater) => None,
                (Equal, Less) => {
                    if self.y + i <= other.y {
                        Some(Self {
                            y: self.y + i,
                            ..self
                        })
                    } else {
                        None
                    }
                }
                (Equal, Equal) => (i == 0).then_some(self),
                (Equal, Greater) => {
                    if other.y + i <= self.y {
                        Some(Self {
                            y: other.y + i,
                            ..self
                        })
                    } else {
                        None
                    }
                }
                (Greater, Less) => None,
                (Greater, Equal) => {
                    if other.x + i <= self.x {
                        Some(Self {
                            x: other.x + i,
                            ..self
                        })
                    } else {
                        None
                    }
                }
                (Greater, Greater) => None,
            },
        )
    }
}

#[derive(Debug)]
struct RockStructure {
    coordinates: Vec<Coordinate>,
}

impl RockStructure {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            separated_list1(tag(" -> "), Coordinate::parse),
            |coordinates| Self { coordinates },
        )(input)
    }

    fn iter_coordinates(&self) -> impl Iterator<Item = Coordinate> + '_ {
        self.coordinates
            .iter()
            .scan(None, |state, &coord| {
                let previous_state = state.unwrap_or(coord);
                *state = Some(coord);
                Some(coord.iter_to_other(previous_state))
            })
            .flatten()
    }
}

#[derive(Debug)]
struct Cave {
    rocks: BTreeSet<Coordinate>,
    sand: BTreeSet<Coordinate>,
    max_y: u16,
    min_x: u16,
    max_x: u16,
}

impl Cave {
    fn new(rocks: BTreeSet<Coordinate>) -> Self {
        Self {
            max_y: rocks.iter().map(|c| c.y).max().unwrap_or(0),
            min_x: rocks.iter().map(|c| c.x - 1).min().unwrap_or(500),
            max_x: rocks.iter().map(|c| c.x + 1).max().unwrap_or(500),
            rocks,
            sand: BTreeSet::new(),
        }
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        let (rest, rock_structures) = separated_list1(newline, RockStructure::parse)(input)?;
        let rocks = rock_structures
            .iter()
            .flat_map(|rs| rs.iter_coordinates())
            .collect();
        Ok((rest, Self::new(rocks)))
    }

    fn is_blocked(&self, coord: Coordinate) -> bool {
        self.rocks.contains(&coord) || self.sand.contains(&coord)
    }

    fn drop_sand(&mut self) -> bool {
        let mut c = Coordinate { x: 500, y: 0 };
        while c.y <= self.max_y {
            match (
                self.is_blocked(Coordinate {
                    x: c.x - 1,
                    y: c.y + 1,
                }),
                self.is_blocked(Coordinate { x: c.x, y: c.y + 1 }),
                self.is_blocked(Coordinate {
                    x: c.x + 1,
                    y: c.y + 1,
                }),
            ) {
                (_, false, _) => c = Coordinate { x: c.x, y: c.y + 1 },
                (false, true, _) => {
                    c = Coordinate {
                        x: c.x - 1,
                        y: c.y + 1,
                    }
                }
                (true, true, false) => {
                    c = Coordinate {
                        x: c.x + 1,
                        y: c.y + 1,
                    }
                }
                (true, true, true) => {
                    self.sand.insert(c);
                    return true;
                }
            }
        }
        false
    }

    fn draw(&self, mut w: impl std::io::Write) -> std::io::Result<()> {
        for y in 0..=self.max_y {
            for x in self.min_x..=self.max_x {
                let c = Coordinate { x, y };
                write!(
                    w,
                    "{}",
                    if self.rocks.contains(&c) {
                        '#'
                    } else if self.sand.contains(&c) {
                        'o'
                    } else if x == 500 && y == 0 {
                        'p'
                    } else {
                        ' '
                    }
                )?;
            }
            writeln!(w)?;
        }
        Ok(())
    }
}

pub fn part_one(input: &str) -> usize {
    let mut cave = Cave::parse(input).unwrap().1;
    let mut counter = 0;
    while cave.drop_sand() {counter += 1}
    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_one() {
        assert_eq!(part_one(include_str!("../example.txt")), 24);
    }

    #[test]
    fn challenge_part_one() {
        assert_eq!(part_one(include_str!("../challenge.txt")), 737);
    }

}
