use std::{collections::BTreeSet, mem::take};

struct Valley {
    width: usize,
    height: usize,
    /// blizzards moving up. A vec of columns
    up_blizzards: Vec<Vec<bool>>,
    /// blizzards moving right. A vec of rows
    right_blizzards: Vec<Vec<bool>>,
    /// blizzards moving down. A vec of columns
    down_blizzards: Vec<Vec<bool>>,
    /// blizzards moving left. A vec of rows
    left_blizzards: Vec<Vec<bool>>,
}

impl Valley {
    fn new(input: &str) -> Self {
        let width = input.lines().next().unwrap().chars().count();
        let height = input.lines().filter(|line| !line.is_empty()).count();

        let mut valley = Self {
            width,
            height,
            up_blizzards: vec![vec![false; height]; width],
            right_blizzards: vec![vec![false; width]; height],
            down_blizzards: vec![vec![false; height]; width],
            left_blizzards: vec![vec![false; width]; height],
        };
        for (y, row) in input.lines().enumerate() {
            for (x, c) in row.chars().enumerate() {
                match c {
                    '^' => valley.up_blizzards[x][y] = true,
                    '>' => valley.right_blizzards[y][x] = true,
                    'v' => valley.down_blizzards[x][y] = true,
                    '<' => valley.left_blizzards[y][x] = true,
                    _ => {}
                }
            }
        }
        valley
    }

    fn is_free(&self, x: usize, y: usize, time: usize) -> bool {
        let v_width = self.width - 2;
        let v_height = self.height - 2;
        if (y == 0 && x == 1) || (y + 1 == self.height && x + 2 == self.width) {
            return true;
        }
        if x == 0 || x + 1 == self.width || y == 0 || y + 1 == self.height {
            return false;
        }
        if self.down_blizzards[x][(y + v_height - time % v_height - 1) % v_height + 1] {
            return false;
        }
        if self.left_blizzards[y][(x + time - 1) % v_width + 1] {
            return false;
        }
        if self.up_blizzards[x][(y + time - 1) % v_height + 1] {
            return false;
        }
        if self.right_blizzards[y][(x + v_width - time % v_width - 1) % v_width + 1] {
            return false;
        }
        true
    }

    fn journey_time(
        &self,
        start: (usize, usize),
        goal: (usize, usize),
        start_time: usize,
    ) -> usize {
        let mut time = start_time;
        let mut possible_tiles = BTreeSet::<(usize, usize)>::new();
        possible_tiles.insert(start);
        loop {
            time += 1;
            if possible_tiles.is_empty() {
                panic!("Ran out of moves");
            }
            if time > 1000 {
                panic!("over 1000 cycles");
            }

            for &(x, y) in take(&mut possible_tiles).iter() {
                if y > 0 && self.is_free(x, y - 1, time) {
                    possible_tiles.insert((x, y - 1));
                }
                if self.is_free(x + 1, y, time) {
                    possible_tiles.insert((x + 1, y));
                }
                if y + 1 < self.height && self.is_free(x, y + 1, time) {
                    possible_tiles.insert((x, y + 1));
                }
                if x > 0 && self.is_free(x - 1, y, time) {
                    possible_tiles.insert((x - 1, y));
                }
                if self.is_free(x, y, time) {
                    possible_tiles.insert((x, y));
                }
            }

            #[cfg(feature = "print")]
            {
                for y in 0..self.height {
                    for x in 0..self.width {
                        print!(
                            "{}",
                            match (self.is_free(x, y, time), possible_tiles.contains(&(x, y))) {
                                (true, true) => 'E',
                                (true, false) => ' ',
                                (false, true) => panic!(),
                                (false, false) => '#',
                            }
                        );
                    }
                    println!();
                }
                println!();
            }
            if possible_tiles.contains(&goal) {
                return time;
            }
        }
    }
}

pub fn part_one(input: &str) -> usize {
    let valley = Valley::new(input);
    valley.journey_time((1, 0), (valley.width - 2, valley.height - 1), 0)
}

pub fn part_two(input: &str) -> usize {
    let valley = Valley::new(input);
    let start = (1, 0);
    let goal = (valley.width - 2, valley.height - 1);

    let there = valley.journey_time(start, goal, 0);
    let back = valley.journey_time(goal, start, there);
    let again = valley.journey_time(start, goal, back);
    again
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_one() {
        assert_eq!(part_one(include_str!("../example.txt")), 18);
    }

    #[test]
    fn challenge_part_one() {
        assert_eq!(part_one(include_str!("../challenge.txt")), 277);
    }

    #[test]
    fn example_part_two() {
        assert_eq!(part_two(include_str!("../example.txt")), 54);
    }

    #[test]
    fn challenge_part_two() {
        assert_eq!(part_two(include_str!("../challenge.txt")), 877);
    }
}
