use std::collections::BinaryHeap;

pub fn part_one(input: &str) -> u64 {
    let mut max = 0;

    let mut current = 0;
    for line in input.lines() {
        if line.is_empty() {
            current = 0;
        } else {
            current += line.parse::<u64>().unwrap();
            max = u64::max(max, current);
        }
    }

    max
}

pub fn part_two(input: &str) -> u64 {
    let mut heap = BinaryHeap::new();

    let mut current = 0;
    for line in input.lines() {
        if line.is_empty() {
            heap.push(current);
            current = 0;
        } else {
            current += line.parse::<u64>().unwrap();
        }
    }
    heap.push(current);

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
