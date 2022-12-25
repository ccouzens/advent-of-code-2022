fn snafu_to_num(snafu: &str) -> i64 {
    let mut num = 0;
    for s in snafu.chars() {
        num = num * 5
            + match s {
                '2' => 2,
                '1' => 1,
                '-' => -1,
                '=' => -2,
                _ => 0,
            };
    }
    num
}

fn num_to_snafu(mut num: i64) -> String {
    let mut digits = Vec::<char>::new();
    while num != 0 {
        let rem = num % 5;
        digits.push(match rem {
            1 => '1',
            2 => '2',
            3 => '=',
            4 => '-',
            _ => '0',
        });

        num =
            (num - match rem {
                1 => 1,
                2 => 2,
                3 => -2,
                4 => -3,
                _ => 0,
            }) / 5;
    }
    if digits.is_empty() {
        String::from("0")
    } else {
        digits.iter().rev().collect()
    }
}

fn parse_list(input: &str) -> Vec<i64> {
    input.lines().map(snafu_to_num).collect()
}

pub fn part_one(input: &str) -> String {
    let sum = parse_list(input).iter().sum::<i64>();

    num_to_snafu(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_one() {
        assert_eq!(part_one(include_str!("../example.txt")), "2=-1=0");
    }

    #[test]
    fn challenge_part_one() {
        assert_eq!(
            part_one(include_str!("../challenge.txt")),
            "2-20=01--0=0=0=2-120"
        );
    }
}
