use std::iter::zip;

fn parse(input: &str) -> Vec<i64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn mix(input: &[i64], order: &mut Vec<i64>) -> Vec<i64> {
    for (i, &num) in zip(0.., input.iter()) {
        let current_position = order.iter().position(|&o| o == i).unwrap() as i64;
        let new_position = (current_position + num).rem_euclid(input.len() as i64 - 1);
        if current_position != new_position {
            order.remove(current_position as usize);
            order.insert(new_position as usize, i);
        }
    }

    order.iter().map(|&i| input[i as usize]).collect()
}

pub fn part_one(input: &str) -> i64 {
    let nums = parse(input);
    let mut order = (0..nums.len() as i64).collect::<Vec<_>>();
    let mixed = mix(&nums, &mut order);

    let p = mixed.iter().position(|&n| n == 0).unwrap();
    mixed[(p + 1000) % mixed.len()]
        + mixed[(p + 2000) % mixed.len()]
        + mixed[(p + 3000) % mixed.len()]
}

pub fn part_two(input: &str) -> i64 {
    let nums: Vec<i64> = parse(input).iter().map(|n| n * 811589153).collect();
    let mut order = (0..nums.len() as i64).collect::<Vec<_>>();
    let mut mixed = Vec::new();
    for _ in 0..10 {
        mixed = mix(&nums, &mut order);
    }

    let p = mixed.iter().position(|&n| n == 0).unwrap();
    mixed[(p + 1000) % mixed.len()]
        + mixed[(p + 2000) % mixed.len()]
        + mixed[(p + 3000) % mixed.len()]
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example_part_one() {
        assert_eq!(part_one(include_str!("../example.txt")), 3);
    }

    #[test]
    fn challenge_part_one() {
        assert_eq!(part_one(include_str!("../challenge.txt")), 8028);
    }

    #[test]
    fn example_part_two() {
        assert_eq!(part_two(include_str!("../example.txt")), 1623178306);
    }

    #[test]
    fn challenge_part_two() {
        assert_eq!(part_two(include_str!("../challenge.txt")), 8798438007673);
    }
}
