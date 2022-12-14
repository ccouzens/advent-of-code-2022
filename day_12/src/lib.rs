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

fn traverse_backwards(
    height_map: &HeightMap,
    reached_start: impl Fn(usize, usize, u8) -> bool,
) -> Result<usize, &'static str> {
    let mut visited: BTreeSet<(usize, usize)> = [height_map.end].iter().cloned().collect();
    let mut recently_visited: Vec<(usize, usize)> = visited.iter().cloned().collect();
    let mut steps = 0;
    loop {
        steps += 1;
        if recently_visited.is_empty() {
            return Err("No where left to walk");
        }
        for &(x, y) in take(&mut recently_visited).iter() {
            if let Some(&height) = height_map.heights.get(y).and_then(|row| row.get(x)) {
                for &neighbour in [
                    (x, y.wrapping_sub(1)),
                    (x + 1, y),
                    (x, y + 1),
                    (x.wrapping_sub(1), y),
                ]
                .iter()
                {
                    if let Some(&neighbour_height) = height_map
                        .heights
                        .get(neighbour.1)
                        .and_then(|row| row.get(neighbour.0))
                    {
                        if neighbour_height + 1 >= height && visited.insert(neighbour) {
                            recently_visited.push(neighbour);
                            if reached_start(neighbour.0, neighbour.1, neighbour_height) {
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
    let height_map = HeightMap::try_from(input)?;

    traverse_backwards(&height_map, |x, y, _| {
        x == height_map.start.0 && y == height_map.start.1
    })
}

pub fn part_two(input: &str) -> Result<usize, &'static str> {
    let height_map = HeightMap::try_from(input)?;

    traverse_backwards(&height_map, |_, _, height| height == 0)
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
