use std::collections::BTreeSet;

struct Backpack<'a> {
    a: &'a [u8],
    b: &'a [u8],
}

fn item_priority(item: u8) -> u8 {
    match item {
        b'a'..=b'z' => item - b'a' + 1,
        b'A'..=b'Z' => item - b'A' + 27,
        _ => 0,
    }
}

impl<'a> Backpack<'a> {
    fn new(input: &'a str) -> Self {
        let input = input.as_bytes();
        let (a, b) = input.split_at(input.len() / 2);
        Self { a, b }
    }

    fn duplicated_item_priority(&self) -> Option<u64> {
        self.a
            .iter()
            .collect::<BTreeSet<_>>()
            .intersection(&self.b.iter().collect())
            .next()
            .map(|&&item| item_priority(item) as u64)
    }
}

pub fn part_one(input: &str) -> u64 {
    input
        .lines()
        .filter_map(|line| Backpack::new(line).duplicated_item_priority())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_one() {
        assert_eq!(part_one(include_str!("../example.txt")), 157);
    }

    #[test]
    fn challenge_part_one() {
        assert_eq!(part_one(include_str!("../challenge.txt")), 7903);
    }

    // #[test]
    // fn example_part_two() {
    //     assert_eq!(part_two(include_str!("../example_1.txt")), 12);
    // }

    // #[test]
    // fn challenge_part_two() {
    //     assert_eq!(part_two(include_str!("../challenge_1.txt")), 13726);
    // }
}
