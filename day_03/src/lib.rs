use std::collections::BTreeSet;

fn item_priority(item: u8) -> u64 {
    (match item {
        b'a'..=b'z' => item - b'a' + 1,
        b'A'..=b'Z' => item - b'A' + 27,
        _ => 0,
    }) as u64
}

pub fn part_one(input: &str) -> u64 {
    input
        .lines()
        .filter_map(|backpack| {
            let (a, b) = backpack.as_bytes().split_at(backpack.len() / 2);
            a.iter()
                .collect::<BTreeSet<_>>()
                .intersection(&b.iter().collect())
                .map(|&&item| item_priority(item))
                .next()
        })
        .sum()
}

pub fn part_two(input: &str) -> u64 {
    let mut backpacks_iter = input.lines();
    let mut sum = 0;

    while let (Some(a), Some(b), Some(c)) = (
        backpacks_iter.next(),
        backpacks_iter.next(),
        backpacks_iter.next(),
    ) {
        sum += a
            .bytes()
            .collect::<BTreeSet<_>>()
            .intersection(&b.bytes().collect())
            .cloned()
            .collect::<BTreeSet<_>>()
            .intersection(&c.bytes().collect())
            .cloned()
            .map(item_priority)
            .next()
            .unwrap_or(0);
    }

    sum
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

    #[test]
    fn example_part_two() {
        assert_eq!(part_two(include_str!("../example.txt")), 70);
    }

    #[test]
    fn challenge_part_two() {
        assert_eq!(part_two(include_str!("../challenge.txt")), 2548);
    }
}
