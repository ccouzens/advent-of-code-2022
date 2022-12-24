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
}

pub fn part_one(input: &str) -> usize {
    let mut cycle = 0;
    let valley = Valley::new(input);
    let mut possible_tiles = BTreeSet::<(usize, usize)>::new();
    possible_tiles.insert((1, 0));
    loop {
        cycle += 1;
        if possible_tiles.is_empty() {
            panic!("Ran out of moves");
        }
        if cycle > 1000 {
            panic!("over 100 cycles");
        }

        for &(x, y) in take(&mut possible_tiles).iter() {
            if y > 0 && valley.is_free(x, y - 1, cycle) {
                possible_tiles.insert((x, y - 1));
            }
            if valley.is_free(x + 1, y, cycle) {
                possible_tiles.insert((x + 1, y));
            }
            if valley.is_free(x, y + 1, cycle) {
                possible_tiles.insert((x, y + 1));
                if y + 2 == valley.height {
                    return cycle;
                }
            }
            if x > 0 && valley.is_free(x - 1, y, cycle) {
                possible_tiles.insert((x - 1, y));
            }
            if valley.is_free(x, y, cycle) {
                possible_tiles.insert((x, y));
            }
        }

        #[cfg(feature = "print")]
        {
            for y in 0..valley.height {
                for x in 0..valley.width {
                    print!(
                        "{}",
                        match (
                            valley.is_free(x, y, cycle),
                            possible_tiles.contains(&(x, y))
                        ) {
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
    }
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
}
