use std::{collections::BTreeSet, ops::RangeInclusive};

use nom::{
    bytes::{complete::tag, streaming::take_while1},
    character::complete::newline,
    combinator::{iterator, map, map_res, ParserIterator},
    sequence::{terminated, tuple},
    IResult,
};

#[derive(Debug, Clone, Copy)]
struct Location {
    x: i64,
    y: i64,
}

impl Location {
    fn parse_nom(input: &str) -> IResult<&str, Self> {
        fn parse_num(input: &str) -> IResult<&str, i64> {
            map_res(
                take_while1(|c: char| c == '-' || ('0'..='9').contains(&c)),
                str::parse,
            )(input)
        }
        map(
            tuple((tag("x="), &parse_num, tag(", y="), &parse_num)),
            |(_, x, _, y)| Self { x, y },
        )(input)
    }

    fn manhatten_distance(&self, other: &Self) -> u64 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    fn manhatten_points_at_row(&self, distance: u64, row: i64) -> RangeInclusive<i64> {
        let vertical_distance = self.y.abs_diff(row);
        let horizontal_distance: i64 = TryInto::<i64>::try_into(distance).unwrap()
            - TryInto::<i64>::try_into(vertical_distance).unwrap(); // can be negative, which would give a empty range
        (self.x - horizontal_distance)..=(self.x + horizontal_distance)
    }
}

#[derive(Debug)]
struct Sensor {
    location: Location,
    beacon: Location,
}

impl Sensor {
    fn parse_nom(input: &str) -> IResult<&str, Self> {
        map(
            tuple((
                tag("Sensor at "),
                Location::parse_nom,
                tag(": closest beacon is at "),
                Location::parse_nom,
            )),
            |(_, location, _, beacon)| Self { location, beacon },
        )(input)
    }

    fn parse_all_iterator<'a>(
        input: &'a str,
    ) -> ParserIterator<
        &'a str,
        nom::error::Error<&'a str>,
        impl FnMut(&'a str) -> IResult<&'a str, Self>,
    > {
        iterator(input, terminated(Self::parse_nom, newline))
    }

    fn beacon_distance(&self) -> u64 {
        self.location.manhatten_distance(&self.beacon)
    }
}

fn normalize_ranges(mut ranges: Vec<RangeInclusive<i64>>) -> Vec<RangeInclusive<i64>> {
    ranges.sort_by_key(|r| *r.start());
    let mut normalized: Vec<RangeInclusive<i64>> = Vec::new();
    for new_range in ranges.drain(..) {
        if new_range.is_empty() {
            continue;
        }
        if let Some(normalized_range) = normalized.last_mut() {
            if new_range.start() <= normalized_range.end() {
                *normalized_range =
                    *normalized_range.start()..=i64::max(*normalized_range.end(), *new_range.end())
            } else {
                normalized.push(new_range);
            }
        } else {
            normalized.push(new_range);
        }
    }
    normalized
}

pub fn part_one(input: &str, row: i64) -> usize {
    let mut covered_ranges = Vec::new();
    let mut beacons_in_row = BTreeSet::new();
    for sensor in &mut Sensor::parse_all_iterator(input) {
        covered_ranges.push(
            sensor
                .location
                .manhatten_points_at_row(sensor.beacon_distance(), row),
        );
        if sensor.beacon.y == row {
            beacons_in_row.insert(sensor.beacon.x);
        }
    }
    let covered_ranges = normalize_ranges(covered_ranges);

    covered_ranges
        .iter()
        .map(|r| r.clone().count())
        .sum::<usize>()
        - beacons_in_row.len()
}

pub fn part_two(input: &str, search_limit: i64) -> i64 {
    let sensors: Vec<Sensor> = Sensor::parse_all_iterator(input).collect();
    for row in 0..=search_limit {
        let covered_ranges = sensors
            .iter()
            .map(|s| s.location.manhatten_points_at_row(s.beacon_distance(), row))
            .collect();
        let covered_ranges = normalize_ranges(covered_ranges);
        for r in covered_ranges.iter() {
            if (-1..search_limit).contains(r.end()) {
                return (*r.end() + 1) * 4000000 + row;
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_one() {
        assert_eq!(part_one(include_str!("../example.txt"), 10), 26);
    }

    #[test]
    fn challenge_part_one() {
        assert_eq!(part_one(include_str!("../challenge.txt"), 2000000), 4793062);
    }

    #[test]
    fn example_part_two() {
        assert_eq!(part_two(include_str!("../example.txt"), 20), 56000011);
    }

    #[test]
    fn challenge_part_two() {
        assert_eq!(
            part_two(include_str!("../challenge.txt"), 4000000),
            10826395253551
        );
    }
}
