fn unique_window_index(input: &str, size: usize) -> Option<usize> {
    let input: Vec<char> = input.chars().collect();
    let mut index = 0;
    while let Some(window) = input.get(index..index + size) {
        if window
            .iter()
            .enumerate()
            .all(|(i, v)| window.iter().skip(i + 1).all(|w| w != v))
        {
            return Some(index + size);
        }
        index += 1;
    }
    None
}

pub fn part_one(input: &str) -> Option<usize> {
    unique_window_index(input, 4)
}

pub fn part_two(input: &str) -> Option<usize> {
    unique_window_index(input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLES: &[&str] = &[
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
        "bvwbjplbgvbhsrlpgdmjqwftvncz",
        "nppdvjthqldpwncqszvftbrmjlhg",
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
    ];

    #[test]
    fn part_one_examples() {
        assert_eq!(part_one(EXAMPLES[0]), Some(7));
        assert_eq!(part_one(EXAMPLES[1]), Some(5));
        assert_eq!(part_one(EXAMPLES[2]), Some(6));
        assert_eq!(part_one(EXAMPLES[3]), Some(10));
        assert_eq!(part_one(EXAMPLES[4]), Some(11));
    }

    #[test]
    fn part_one_challenge() {
        assert_eq!(part_one(include_str!("../challenge.txt")), Some(1892));
    }

    #[test]
    fn part_two_examples() {
        assert_eq!(part_two(EXAMPLES[0]), Some(19));
        assert_eq!(part_two(EXAMPLES[1]), Some(23));
        assert_eq!(part_two(EXAMPLES[2]), Some(23));
        assert_eq!(part_two(EXAMPLES[3]), Some(29));
        assert_eq!(part_two(EXAMPLES[4]), Some(26));
    }

    #[test]
    fn part_two_challenge() {
        assert_eq!(part_two(include_str!("../challenge.txt")), Some(2313));
    }
}
