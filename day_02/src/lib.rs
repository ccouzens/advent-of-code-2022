enum RPSMove {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Draw,
    Lose,
}

impl RPSMove {
    fn new(input: &str) -> Option<Self> {
        match input {
            "A" => Some(Self::Rock),
            "B" => Some(Self::Paper),
            "C" => Some(Self::Scissors),
            "X" => Some(Self::Rock),
            "Y" => Some(Self::Paper),
            "Z" => Some(Self::Scissors),
            _ => None,
        }
    }
}

struct Round {
    opponent: RPSMove,
    response: RPSMove,
}

impl Round {
    fn new(input: &str) -> Option<Self> {
        let (a, b) = input.split_once(' ')?;
        Some(Self {
            opponent: RPSMove::new(a)?,
            response: RPSMove::new(b)?,
        })
    }

    fn outcome(&self) -> Outcome {
        use Outcome::*;
        use RPSMove::*;
        match (&self.opponent, &self.response) {
            (Rock, Rock) => Draw,
            (Rock, Paper) => Win,
            (Rock, Scissors) => Lose,
            (Paper, Rock) => Lose,
            (Paper, Paper) => Draw,
            (Paper, Scissors) => Win,
            (Scissors, Rock) => Win,
            (Scissors, Paper) => Lose,
            (Scissors, Scissors) => Draw,
        }
    }

    fn score(&self) -> u64 {
        (match self.outcome() {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }) + match self.response {
            RPSMove::Rock => 1,
            RPSMove::Paper => 2,
            RPSMove::Scissors => 3,
        }
    }
}

pub fn part_one(input: &str) -> u64 {
    input
        .lines()
        .map(Round::new)
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
