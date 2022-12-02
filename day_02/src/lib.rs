// Rock: 0
// Paper: 1
// Scissors: 2

// Lose: 0
// Draw: 1
// Win: 2

struct Round {
    theirs: u8,
    ours: u8,
}

impl Round {
    fn new_with_their_move(input: &[u8]) -> Option<Self> {
        Some(Self {
            theirs: input.first()? - b'A',
            ours: input.get(2)? - b'X',
        })
    }

    fn new_with_round_outcome(input: &[u8]) -> Option<Self> {
        let theirs = input.first()? - b'A';
        let outcome = input.get(2)? - b'X';
        let ours = (outcome + 2 + theirs) % 3;
        Some(Self { ours, theirs })
    }

    fn score(&self) -> u64 {
        ((self.ours + 1) + ((self.ours + 4 - self.theirs) % 3) * 3) as u64
    }
}

pub fn part_one(input: &str) -> u64 {
    input
        .lines()
        .filter_map(|line| Round::new_with_their_move(line.as_bytes()))
        .map(|r| Round::score(&r))
        .sum()
}

pub fn part_two(input: &str) -> u64 {
    input
        .lines()
        .filter_map(|line| Round::new_with_round_outcome(line.as_bytes()))
        .map(|r| Round::score(&r))
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

    #[test]
    fn example_part_two() {
        assert_eq!(part_two(include_str!("../example_1.txt")), 12);
    }

    #[test]
    fn challenge_part_two() {
        assert_eq!(part_two(include_str!("../challenge_1.txt")), 13726);
    }
}
