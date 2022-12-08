#[derive(Debug)]
struct Forest {
    width: usize,
    heights: Vec<u8>,
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

struct TreesIter<'a> {
    forest: &'a Forest,
    direction: Direction,
    index: usize,
}

impl<'a> Iterator for TreesIter<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.index;
        let w = self.forest.width;
        let y = i / w;
        let x = i % w;
        let new_index = match self.direction {
            Direction::North => {
                if y == 0 {
                    None
                } else {
                    Some(i - w)
                }
            }
            Direction::East => {
                if x + 1 == w {
                    None
                } else {
                    Some(i + 1)
                }
            }
            Direction::South => {
                if y + 1 == w {
                    None
                } else {
                    Some(i + w)
                }
            }
            Direction::West => {
                if x == 0 {
                    None
                } else {
                    Some(i - 1)
                }
            }
        };
        match new_index {
            None => None,
            Some(i) => {
                self.index = i;
                Some(self.forest.heights[i])
            }
        }
    }
}

impl Forest {
    fn new_from_str(input: &str) -> Self {
        let mut width = 0;
        let mut heights = Vec::new();
        for line in input.lines() {
            heights.extend(line.bytes().map(|d| d - b'0'));
            width += 1;
        }

        Forest { width, heights }
    }

    fn line_of_sight(&self, index: usize, direction: Direction) -> impl Iterator<Item = u8> + '_ {
        TreesIter {
            forest: self,
            index,
            direction,
        }
    }
}

pub fn part_one(input: &str) -> usize {
    let forest = Forest::new_from_str(input);
    forest
        .heights
        .iter()
        .enumerate()
        .filter(|&(i, &h)| {
            DIRECTIONS.iter().any(|&d| {
                forest
                    .line_of_sight(i, d)
                    .max()
                    .map(|m| h > m)
                    .unwrap_or(true)
            })
        })
        .count()
}

pub fn part_two(input: &str) -> usize {
    let forest = Forest::new_from_str(input);
    forest
        .heights
        .iter()
        .enumerate()
        .map(|(i, &h)| {
            DIRECTIONS.iter().fold(1, |acc, &d| {
                let mut count = 0;
                for other in forest.line_of_sight(i, d) {
                    count += 1;
                    if other >= h {
                        break;
                    }
                }
                acc * count
            })
        })
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_one() {
        assert_eq!(part_one(include_str!("../example.txt")), 21);
    }

    #[test]
    fn challenge_part_one() {
        assert_eq!(part_one(include_str!("../challenge.txt")), 1805);
    }

    #[test]
    fn example_part_two() {
        assert_eq!(part_two(include_str!("../example.txt")), 8);
    }

    #[test]
    fn challenge_part_two() {
        assert_eq!(part_two(include_str!("../challenge.txt")), 444528);
    }
}
