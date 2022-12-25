use std::iter::zip;

fn parse(input: &str) -> Vec<i16> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn mix(input: &[i16]) -> Vec<i16> {
    let mut order = (0..input.len() as i16).collect::<Vec<_>>();

    for (i, &num) in zip(0.., input.iter()) {
        let current_position = order.iter().position(|&o| o == i).unwrap() as i16;
        let new_position = (current_position + num).rem_euclid(input.len() as i16 - 1);
        match current_position.cmp(&new_position) {
            std::cmp::Ordering::Less => {
                order.insert(new_position as usize + 1, i);
                order.remove(current_position as usize);
            }
            std::cmp::Ordering::Equal => {}
            std::cmp::Ordering::Greater => {
                order.remove(current_position as usize);
                order.insert(new_position as usize, i);
            }
        }
    }

    order.iter().map(|&i| input[i as usize]).collect()
}

pub fn part_one(input: &str) -> i16 {
    let nums = mix(&parse(input));
    let p = nums.iter().position(|&n| n == 0).unwrap();
    nums[(p + 1000) % nums.len()] + nums[(p + 2000) % nums.len()] + nums[(p + 3000) % nums.len()]
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
}
