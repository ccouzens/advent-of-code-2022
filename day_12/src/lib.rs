use std::{collections::BTreeSet, mem::take};

#[derive(Debug)]
struct HeightMap {
    heights: Vec<Vec<u8>>,
    start: (usize, usize),
    end: (usize, usize),
}

impl TryFrom<&str> for HeightMap {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut start = None;
        let mut end = None;
        let heights = s
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.bytes()
                    .enumerate()
                    .map(|(x, b)| match b {
                        b'S' => {
                            start = Some((x, y));
                            0
                        }
                        b'E' => {
                            end = Some((x, y));
                            25
                        }
                        _ => b - b'a',
                    })
                    .collect()
            })
            .collect();
        Ok(Self {
            start: start.ok_or("Couldn't find Start")?,
            end: end.ok_or("Couldn't find End")?,
            heights,
        })
    }
}

fn traverse_backwards(input: &str, part_two: bool) -> Result<usize, &'static str> {
    let height_map = HeightMap::try_from(input)?;
    let mut visited_positions: BTreeSet<(usize, usize)> =
        [height_map.end].iter().cloned().collect();
    let mut next_positions = visited_positions.clone();
    let mut steps = 0;
    loop {
        steps += 1;
        if next_positions.is_empty() {
            return Err("No where left to walk");
        }
        for &(x, y) in take(&mut next_positions).iter() {
            if let Some(&height) = height_map.heights.get(y).and_then(|row| row.get(x)) {
                for &neighbour in [
                    (x, y.saturating_sub(1)),
                    (x + 1, y),
                    (x, y + 1),
                    (x.saturating_sub(1), y),
                ]
                .iter()
                {
                    if let Some(&neighbour_height) = height_map
                        .heights
                        .get(neighbour.1)
                        .and_then(|row| row.get(neighbour.0))
                    {
                        if neighbour_height + 1 >= height && visited_positions.insert(neighbour) {
                            next_positions.insert(neighbour);
                            if part_two && neighbour_height == 0 || neighbour == height_map.start {
                                return Ok(steps);
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn part_one(input: &str) -> Result<usize, &'static str> {
    traverse_backwards(input, false)
}

pub fn part_two(input: &str) -> Result<usize, &'static str> {
    traverse_backwards(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_one() {
        assert_eq!(part_one(include_str!("../example.txt")).unwrap(), 31);
    }

    #[test]
    fn challenge_part_one() {
        assert_eq!(part_one(include_str!("../challenge.txt")).unwrap(), 528);
    }
    #[test]
    fn example_part_two() {
        assert_eq!(part_two(include_str!("../example.txt")).unwrap(), 29);
    }

    #[test]
    fn challenge_part_two() {
        assert_eq!(part_two(include_str!("../challenge.txt")).unwrap(), 522);
    }
}
