use nom::{
    character::complete::{char, digit1},
    combinator::{map, map_res},
    sequence::separated_pair,
    IResult,
};

#[derive(Debug)]
struct SectionAssignment(u64, u64);

impl SectionAssignment {
    fn fully_contains(&self, other: &Self) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }
}

fn parse_section_assignments(input: &str) -> IResult<&str, SectionAssignment> {
    map(
        separated_pair(
            map_res(digit1, str::parse),
            char('-'),
            map_res(digit1, str::parse),
        ),
        |p| SectionAssignment(p.0, p.1),
    )(input)
}

fn parse_elf_pair(input: &str) -> IResult<&str, (SectionAssignment, SectionAssignment)> {
    separated_pair(
        parse_section_assignments,
        char(','),
        parse_section_assignments,
    )(input)
}

fn elf_pairs(input: &str) -> impl Iterator<Item = (SectionAssignment, SectionAssignment)> + '_ {
    input.lines().filter_map(|line| match parse_elf_pair(line) {
        Ok(("", (a, b))) => Some((a, b)),
        _ => None,
    })
}

pub fn part_one(input: &str) -> usize {
    elf_pairs(input)
        .filter(|(a, b)| a.fully_contains(b) || b.fully_contains(a))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_one() {
        assert_eq!(part_one(include_str!("../example.txt")), 2);
    }

    #[test]
    fn challenge_part_one() {
        assert_eq!(part_one(include_str!("../challenge.txt")), 584);
    }
}
