use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{anychar, char, digit1, line_ending},
    combinator::{map, map_res},
    multi::{count, fold_many1, many1, separated_list1},
    sequence::{delimited, terminated, tuple},
    IResult,
};

fn parse_crate(input: &str) -> IResult<&str, char> {
    delimited(char('['), anychar, char(']'))(input)
}

fn parse_crate_row(input: &str) -> IResult<&str, Vec<Option<char>>> {
    terminated(
        separated_list1(
            char(' '),
            alt((map(parse_crate, Some), map(tag("   "), |_| None))),
        ),
        line_ending,
    )(input)
}

fn parse_stack_labels(input: &str) -> IResult<&str, usize> {
    terminated(
        fold_many1(
            tuple((
                take_while(|chr| chr == ' '),
                digit1,
                take_while(|chr| chr == ' '),
            )),
            || 0,
            |acc, _| acc + 1,
        ),
        count(line_ending, 2),
    )(input)
}

fn parse_starting_stacks(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    map(
        tuple((many1(parse_crate_row), parse_stack_labels)),
        |(crates_drawing, count)| {
            let mut stacks = vec![vec![]; count];
            for drawing_row in crates_drawing.iter().rev() {
                for (drawn_crate, stack) in drawing_row.iter().zip(stacks.iter_mut()) {
                    if let Some(drawn_crate) = drawn_crate {
                        stack.push(*drawn_crate);
                    }
                }
            }
            stacks
        },
    )(input)
}

#[derive(Debug)]
struct Instruction {
    num: usize,
    from: usize,
    to: usize,
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    map(
        tuple((
            tag("move "),
            map_res(digit1, str::parse),
            tag(" from "),
            map_res(digit1, str::parse),
            tag(" to "),
            map_res(digit1, str::parse),
        )),
        |(_, num, _, from, _, to)| Instruction { num, from, to },
    )(input)
}

fn crane(input: &str, follow_instruction: impl Fn(&mut Vec<Vec<char>>, Instruction)) -> String {
    let (input, mut stacks) = parse_starting_stacks(input).unwrap();

    for instruction_line in input.lines() {
        let instruction = parse_instruction(instruction_line).unwrap().1;
        follow_instruction(&mut stacks, instruction);
    }
    stacks.iter().filter_map(|stack| stack.last()).collect()
}

pub fn part_one(input: &str) -> String {
    crane(
        input,
        |stacks: &mut Vec<Vec<char>>, instruction: Instruction| {
            for _ in 0..instruction.num {
                if let Some(elf_crate) = stacks[instruction.from - 1].pop() {
                    stacks[instruction.to - 1].push(elf_crate);
                }
            }
        },
    )
}

pub fn part_two(input: &str) -> String {
    crane(
        input,
        |stacks: &mut Vec<Vec<char>>, instruction: Instruction| {
            let from_stack = &mut stacks[instruction.from - 1];
            let mut tmp_stack = from_stack.split_off(from_stack.len() - instruction.num);
            stacks[instruction.to - 1].append(&mut tmp_stack);
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_one() {
        assert_eq!(part_one(include_str!("../example.txt")), "CMZ");
    }

    #[test]
    fn challenge_part_one() {
        assert_eq!(part_one(include_str!("../challenge.txt")), "QNHWJVJZW");
    }

    #[test]
    fn example_part_two() {
        assert_eq!(part_two(include_str!("../example.txt")), "MCD");
    }

    #[test]
    fn challenge_part_two() {
        assert_eq!(part_two(include_str!("../challenge.txt")), "BPCZJLFJW");
    }
}
