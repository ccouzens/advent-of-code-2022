use std::collections::{hash_map, HashMap};

#[derive(Clone, Copy)]
struct RP {
    x: usize,
    y: usize,
}

enum Direction {
    Left,
    Right,
}

#[derive(Clone, Copy)]
struct Rock<'a> {
    shape: &'a [RP],
}

const ROCKS: &[Rock] = &[
    // ####
    Rock {
        shape: &[
            RP { x: 0, y: 0 },
            RP { x: 1, y: 0 },
            RP { x: 2, y: 0 },
            RP { x: 3, y: 0 },
        ],
    },
    // .#.
    // ###
    // .#.
    Rock {
        shape: &[
            RP { x: 1, y: 2 },
            RP { x: 0, y: 1 },
            RP { x: 1, y: 1 },
            RP { x: 2, y: 1 },
            RP { x: 1, y: 0 },
        ],
    },
    // ..#
    // ..#
    // ###
    Rock {
        shape: &[
            RP { x: 2, y: 2 },
            RP { x: 2, y: 1 },
            RP { x: 0, y: 0 },
            RP { x: 1, y: 0 },
            RP { x: 2, y: 0 },
        ],
    },
    // #
    // #
    // #
    // #
    Rock {
        shape: &[
            RP { x: 0, y: 3 },
            RP { x: 0, y: 2 },
            RP { x: 0, y: 1 },
            RP { x: 0, y: 0 },
        ],
    },
    // ##
    // ##
    Rock {
        shape: &[
            RP { x: 0, y: 1 },
            RP { x: 1, y: 1 },
            RP { x: 0, y: 0 },
            RP { x: 1, y: 0 },
        ],
    },
];

struct Simulation {
    stopped_rocks: Vec<[bool; 7]>,
    rock_cycle: usize,
    wind_cycle: usize,
    wind_directions: Vec<Direction>,
}

impl Simulation {
    fn new(input: &str) -> Self {
        Self {
            stopped_rocks: Vec::new(),
            rock_cycle: 0,
            wind_cycle: 0,
            wind_directions: input
                .chars()
                .map_while(|c| match c {
                    '<' => Some(Direction::Left),
                    '>' => Some(Direction::Right),
                    _ => None,
                })
                .collect(),
        }
    }

    fn valid_position(
        &self,
        rock: &Rock,
        x_offset: Option<usize>,
        y_offset: Option<usize>,
    ) -> bool {
        let x_offset = match x_offset {
            None => {
                return false;
            }
            Some(x_offset) => x_offset,
        };
        let y_offset = match y_offset {
            None => {
                return false;
            }
            Some(y_offset) => y_offset,
        };
        for s in rock.shape.iter() {
            let x = s.x + x_offset;
            let y = s.y + y_offset;
            if x >= 7 {
                return false;
            }
            match self.stopped_rocks.get(y) {
                None => {}
                Some(row) => {
                    if row.get(x) != Some(&false) {
                        return false;
                    }
                }
            };
        }

        true
    }

    fn solidify_rock(&mut self, rock: &Rock, x_offset: usize, y_offset: usize) {
        for s in rock.shape.iter() {
            let x = s.x + x_offset;
            let y = s.y + y_offset;
            loop {
                match self.stopped_rocks.get_mut(y) {
                    None => self.stopped_rocks.push([false; 7]),
                    Some(row) => {
                        row[x] = true;
                        break;
                    }
                }
            }
        }
    }

    fn drop_rock(&mut self) {
        let rock = ROCKS[self.rock_cycle];
        let mut x_offset: usize = 2;
        let mut y_offset: usize = 3 + self.stopped_rocks.len();
        loop {
            let new_x_offset = match self.wind_directions[self.wind_cycle] {
                Direction::Left => x_offset.checked_sub(1),
                Direction::Right => x_offset.checked_add(1),
            };
            if self.valid_position(&rock, new_x_offset, Some(y_offset)) {
                x_offset = new_x_offset.unwrap();
            }
            self.wind_cycle = (self.wind_cycle + 1) % self.wind_directions.len();
            if self.valid_position(&rock, Some(x_offset), y_offset.checked_sub(1)) {
                y_offset -= 1;
            } else {
                self.solidify_rock(&rock, x_offset, y_offset);
                break;
            }
        }
        self.rock_cycle = (self.rock_cycle + 1) % ROCKS.len();
    }
}

pub fn part_one(input: &str) -> usize {
    let mut simulation = Simulation::new(input);
    for _ in 0..2022 {
        simulation.drop_rock();
    }
    simulation.stopped_rocks.len()
}

pub fn part_two(input: &str) -> usize {
    let mut simulation = Simulation::new(input);
    let mut cycle_detector = HashMap::<(Vec<[bool; 7]>, usize, usize), (usize, u64)>::new();
    let mut counter: u64 = 0;
    let mut cave_height_increase = 0;
    let goal: u64 = 1000000000000;
    let mut jumped = false;
    loop {
        simulation.drop_rock();
        counter += 1;
        if !jumped {
            if let Some(last_rows) = simulation
                .stopped_rocks
                .len()
                .checked_sub(9)
                .and_then(|s| simulation.stopped_rocks.get(s..))
                .map(|s| s.to_vec())
            {
                match cycle_detector.entry((
                    last_rows,
                    simulation.rock_cycle,
                    simulation.wind_cycle,
                )) {
                    hash_map::Entry::Occupied(o) => {
                        let repeats = (goal - counter) / (counter - o.get().1);
                        cave_height_increase +=
                            repeats as usize * (simulation.stopped_rocks.len() - o.get().0);
                        counter += repeats * (counter - o.get().1);
                        jumped = true;
                    }
                    hash_map::Entry::Vacant(v) => {
                        v.insert((simulation.stopped_rocks.len(), counter));
                    }
                }
            }
        }
        if counter == goal {
            return cave_height_increase + simulation.stopped_rocks.len();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_one() {
        assert_eq!(part_one(include_str!("../example.txt")), 3068);
    }

    #[test]
    fn challenge_part_one() {
        assert_eq!(part_one(include_str!("../challenge.txt")), 3048);
    }

    #[test]
    fn example_part_two() {
        assert_eq!(part_two(include_str!("../example.txt")), 1514285714288);
    }

    #[test]
    fn challenge_part_two() {
        assert_eq!(part_two(include_str!("../challenge.txt")), 1504093567249);
    }
}
