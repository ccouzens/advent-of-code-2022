use std::collections::BinaryHeap;

pub fn part_one(input: &str) -> u64 {
    input
        .split("\n\n")
        .map(|elf| elf.lines().map(|line| line.parse().unwrap_or(0)).sum())
        .max()
        .unwrap_or(0)
}

pub fn part_two(input: &str) -> u64 {
    let mut heap = input
        .split("\n\n")
        .map(|elf| elf.lines().map(|line| line.parse().unwrap_or(0)).sum())
        .collect::<BinaryHeap<_>>();

    heap.pop().unwrap_or(0) + heap.pop().unwrap_or(0) + heap.pop().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_one() {
        assert_eq!(part_one(include_str!("../example_1.txt")), 24000);
    }

    #[test]
    fn challenge_part_one() {
        assert_eq!(part_one(include_str!("../challenge_1.txt")), 72602);
    }

    #[test]
    fn example_part_two() {
        assert_eq!(part_two(include_str!("../example_1.txt")), 45000);
    }

    #[test]
    fn challenge_part_two() {
        assert_eq!(part_two(include_str!("../challenge_1.txt")), 207410);
    }
}
