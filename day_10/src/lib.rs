use std::iter::{once, zip};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::line_ending,
    combinator::{iterator, map, map_res, value},
    sequence::{preceded, terminated},
    IResult,
};

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

use Instruction::*;

enum InternalInstruction {
    Noop,
    AddxPart2(i32),
}

impl InternalInstruction {
    fn run(&self, register_x: &mut i32) {
        match self {
            Self::Noop => {}
            Self::AddxPart2(a) => *register_x += a,
        }
    }
}

impl Instruction {
    fn nom_parse(input: &str) -> IResult<&str, Instruction> {
        alt::<_, _, nom::error::Error<&str>, _>((
            value(Noop, tag("noop")),
            map(
                preceded(
                    tag("addx "),
                    map_res(
                        take_while1(|c| c == '-' || ('0'..='9').contains(&c)),
                        str::parse,
                    ),
                ),
                Addx,
            ),
        ))(input)
    }

    fn internal_instruction_iter(self) -> impl Iterator<Item = InternalInstruction> {
        (0..).map_while(move |i| match (i, self) {
            (0, Noop | Addx(_)) => Some(InternalInstruction::Noop),
            (1, Addx(a)) => Some(InternalInstruction::AddxPart2(a)),
            _ => None,
        })
    }
}

fn internal_instruction(input: &str) -> Vec<InternalInstruction> {
    let mut instructions_iterator =
        iterator(input, terminated(Instruction::nom_parse, line_ending));
    (&mut instructions_iterator)
        .flat_map(|i| i.internal_instruction_iter())
        .collect()
}

fn register_x_values(
    internal_instruction: &[InternalInstruction],
) -> impl Iterator<Item = i32> + '_ {
    let mut register_x = 1;
    once(register_x).chain(
        internal_instruction
            .iter()
            .map(move |internal_instruction| {
                internal_instruction.run(&mut register_x);
                register_x
            }),
    )
}

pub fn part_one(input: &str) -> i32 {
    zip(register_x_values(&internal_instruction(input)), 1..)
        .filter_map(|(register_x, cycle)| match cycle {
            20 | 60 | 100 | 140 | 180 | 220 => Some(cycle * register_x),
            _ => None,
        })
        .sum()
}

pub fn part_two(input: &str) -> String {
    let mut output = String::new();
    for (x, cycle) in zip(register_x_values(&internal_instruction(input)), 0..240) {
        let i = cycle % 40;
        output.push(if x - 1 == i || x == i || x + 1 == i {
            '#'
        } else {
            '.'
        });
        if i == 39 {
            output.push('\n');
        }
    }
    output
}

#[cfg(test)]
mod tests {

    use super::*;
    use image::ImageBuffer;

    #[test]
    fn example_part_one() {
        assert_eq!(part_one(include_str!("../example.txt")), 13140);
    }

    #[test]
    fn challenge_part_one() {
        assert_eq!(part_one(include_str!("../challenge.txt")), 16880);
    }

    #[test]
    fn example_part_two() {
        assert_eq!(
            part_two(include_str!("../example.txt")),
            include_str!("../example_answer_part_2.txt")
        );
    }

    #[test]
    fn challenge_part_two() {
        let s = part_two(include_str!("../challenge.txt"));
        let lines: Vec<&[u8]> = s.lines().map(|s| s.as_bytes()).collect();
        let img = ImageBuffer::from_fn(40, 6, |x, y| {
            if lines[y as usize][x as usize] == b'#' {
                image::Luma([0u8])
            } else {
                image::Luma([255u8])
            }
        });
        img.save("challenge_answer_part_2.png").unwrap();
        assert_eq!(&s, include_str!("../challenge_answer_part_2.txt"));
    }
}
