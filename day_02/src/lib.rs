struct Round {
    theirs: u8,
    ours: u8,
}

impl Round {
    fn new(input: &[u8]) -> Option<Self> {
        Some(Self {
            theirs: input.first()? - b'A',
            ours: input.get(2)? - b'X',
        })
    }

    fn score(&self) -> u64 {
        ((self.ours + 1) + ((self.ours + 4 - self.theirs) % 3) * 3) as u64
    }
}

pub fn part_one(input: &str) -> u64 {
    input
        .lines()
        .map(|line| Round::new(line.as_bytes()))
        .map(|r| Round::score(&r.unwrap()))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_one() {
        assert_eq!(part_one(include_str!("../example_1.txt")), 15);
    }

    #[test]
    fn challenge_part_one() {
        assert_eq!(part_one(include_str!("../challenge_1.txt")), 12855);
    }
}
