use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{digit1, newline},
    combinator::{iterator, map, map_res, value},
    sequence::{terminated, tuple},
    IResult,
};

#[derive(Clone, Copy, Debug)]
enum Operation {
    Plus,
    Minus,
    Mul,
    Div,
}

#[derive(Clone, Debug)]
enum Job<'a> {
    Number(i64),
    Dependent {
        a: &'a str,
        op: Operation,
        b: &'a str,
    },
}

impl<'a> Job<'a> {
    fn get_value(&self, troop: &Troop) -> Option<i64> {
        match self {
            Job::Number(n) => Some(*n),
            Job::Dependent { a, op, b } => {
                let a = troop.monkeys.get(a)?.get_value(troop)?;
                let b = troop.monkeys.get(b)?.get_value(troop)?;
                match op {
                    Operation::Plus => Some(a + b),
                    Operation::Minus => Some(a - b),
                    Operation::Mul => Some(a * b),
                    Operation::Div => Some(a / b),
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
struct MonkeyJob<'a> {
    name: &'a str,
    job: Job<'a>,
}

impl<'a> MonkeyJob<'a> {
    fn parse_nom(input: &'a str) -> IResult<&'a str, Self> {
        let name_cond = |c: char| ('a'..='z').contains(&c);
        map(
            tuple((
                take_while1(name_cond),
                tag(": "),
                alt((
                    map_res(digit1, |c: &str| c.parse().map(Job::Number)),
                    map(
                        tuple((
                            take_while1(name_cond),
                            alt((
                                value(Operation::Plus, tag(" + ")),
                                value(Operation::Minus, tag(" - ")),
                                value(Operation::Mul, tag(" * ")),
                                value(Operation::Div, tag(" / ")),
                            )),
                            take_while1(name_cond),
                        )),
                        |(a, op, b)| Job::Dependent { a, op, b },
                    ),
                )),
            )),
            |(name, _, job)| MonkeyJob { name, job },
        )(input)
    }
}

#[derive(Clone, Debug)]
struct Troop<'a> {
    monkeys: HashMap<&'a str, Job<'a>>,
}

impl<'a> Troop<'a> {
    fn new(input: &'a str) -> Self {
        let mut monkeys = HashMap::new();
        for monkey_job in &mut iterator(input, terminated(MonkeyJob::parse_nom, newline)) {
            monkeys.insert(monkey_job.name, monkey_job.job);
        }
        Self { monkeys }
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let troop = Troop::new(input);
    let root = troop.monkeys.get("root")?;
    root.get_value(&troop)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_one() {
        assert_eq!(part_one(include_str!("../example.txt")), Some(152));
    }

    #[test]
    fn challenge_part_one() {
        assert_eq!(
            part_one(include_str!("../challenge.txt")),
            Some(49288254556480)
        );
    }
}
