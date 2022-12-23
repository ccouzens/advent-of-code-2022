use std::{
    collections::{BTreeMap, BTreeSet},
    iter::zip,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Position {
    x: i16,
    y: i16,
}

impl Position {
    fn proposed_move(&self, grove: &Grove) -> Position {
        let mut free = [[true; 3]; 3];
        for (y, row) in zip(-1.., free.iter_mut()) {
            for (x, tile) in zip(-1.., row.iter_mut()) {
                if x != 0 || y != 0 {
                    *tile = !grove.elves.contains(&Position {
                        x: self.x + x,
                        y: self.y + y,
                    })
                }
            }
        }
        if free.iter().all(|row| row.iter().all(|&tile| tile)) {
            return *self;
        }
        for i in 0..4 {
            match (grove.cycle + i) % 4 {
                0 => {
                    // North
                    if free[0][0] && free[0][1] && free[0][2] {
                        return Position {
                            x: self.x,
                            y: self.y - 1,
                        };
                    }
                }
                1 => {
                    // South
                    if free[2][0] && free[2][1] && free[2][2] {
                        return Position {
                            x: self.x,
                            y: self.y + 1,
                        };
                    }
                }
                2 => {
                    // West
                    if free[0][0] && free[1][0] && free[2][0] {
                        return Position {
                            x: self.x - 1,
                            y: self.y,
                        };
                    }
                }
                3 => {
                    // East
                    if free[0][2] && free[1][2] && free[2][2] {
                        return Position {
                            x: self.x + 1,
                            y: self.y,
                        };
                    }
                }
                _ => {}
            }
        }
        *self
    }
}

#[derive(Debug, Default)]
struct Grove {
    elves: BTreeSet<Position>,
    cycle: u8,
}

impl Grove {
    fn new(input: &str) -> Self {
        let mut x = 0;
        let mut y = 1;
        let mut grove = Self::default();
        for c in input.chars() {
            x += 1;
            match c {
                '#' => {
                    grove.elves.insert(Position { x, y });
                }
                '\n' => {
                    y += 1;
                    x = 0;
                }
                _ => {}
            }
        }
        grove
    }

    fn empty_tiles(&self) -> usize {
        let min_x = self.elves.iter().map(|e| e.x).min().unwrap_or(0);
        let min_y = self.elves.iter().map(|e| e.y).min().unwrap_or(0);
        let max_x = self.elves.iter().map(|e| e.x).max().unwrap_or(0);
        let max_y = self.elves.iter().map(|e| e.y).max().unwrap_or(0);
        (min_x..=max_x).len() * (min_y..=max_y).len() - self.elves.len()
    }

    fn follow_round(&mut self) {
        let mut proposed_position_counts = BTreeMap::<Position, usize>::new();
        let mut proposed_moves = BTreeMap::<Position, Position>::new();
        for &elf in self.elves.iter() {
            let proposed_position = elf.proposed_move(self);
            *proposed_position_counts
                .entry(proposed_position)
                .or_default() += 1;
            proposed_moves.insert(elf, proposed_position);
        }
        self.elves = Default::default();
        for (&original, &proposed) in proposed_moves.iter() {
            self.elves.insert(
                if proposed_position_counts.get(&proposed).cloned() == Some(1) {
                    proposed
                } else {
                    original
                },
            );
        }
        self.cycle = (self.cycle + 1) % 4;
    }
}

pub fn part_one(input: &str) -> usize {
    let mut grove = Grove::new(input);
    for _ in 0..10 {
        grove.follow_round();
    }
    grove.empty_tiles()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_one() {
        assert_eq!(part_one(include_str!("../example.txt")), 110);
    }

    #[test]
    fn challenge_part_one() {
        assert_eq!(part_one(include_str!("../challenge.txt")), 4068);
    }
}
