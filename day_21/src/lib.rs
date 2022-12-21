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
    Eq,
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
                    Operation::Eq => None,
                }
            }
        }
    }

    fn missing_value(&self, troop: &Troop, value: i64) -> Option<(i64, &str)> {
        match self {
            Job::Number(_) => None,
            &Job::Dependent { a, op, b } => {
                let a_val = troop.monkeys.get(a).and_then(|j| j.get_value(troop));
                let b_val = troop.monkeys.get(b).and_then(|j| j.get_value(troop));
                match (a_val, op, b_val) {
                    (Some(_), _, Some(_)) => None,
                    (None, _, None) => None,
                    (Some(a_val), Operation::Plus, None) => Some((value - a_val, b)),
                    (Some(a_val), Operation::Minus, None) => Some((a_val - value, b)),
                    (Some(a_val), Operation::Div, None) => Some((a_val / value, b)),
                    (Some(a_val), Operation::Mul, None) => Some((value / a_val, b)),
                    (Some(a_val), Operation::Eq, None) => Some((a_val, b)),
                    (None, Operation::Plus, Some(b_val)) => Some((value - b_val, a)),
                    (None, Operation::Minus, Some(b_val)) => Some((value + b_val, a)),
                    (None, Operation::Div, Some(b_val)) => Some((value * b_val, a)),
                    (None, Operation::Mul, Some(b_val)) => Some((value / b_val, a)),
                    (None, Operation::Eq, Some(b_val)) => Some((b_val, a)),
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

pub fn part_two(input: &str) -> Option<i64> {
    let mut troop = Troop::new(input);
    troop.monkeys.remove("humn");
    if let &Job::Dependent { a, b, .. } = troop.monkeys.get("root")? {
        let a_val = troop.monkeys.get(a).and_then(|j| j.get_value(&troop));
        let b_val = troop.monkeys.get(b).and_then(|j| j.get_value(&troop));
        let mut job = &Job::Dependent {
            a,
            op: Operation::Eq,
            b,
        };
        let mut value = a_val.or(b_val)?;
        loop {
            let value_monkey = job.missing_value(&troop, value)?;
            value = value_monkey.0;
            let monkey = value_monkey.1;
            if monkey == "humn" {
                break Some(value);
            } else {
                job = troop.monkeys.get(monkey)?;
            }
        }
    } else {
        None
    }
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

    #[test]
    fn example_part_two() {
        assert_eq!(part_two(include_str!("../example.txt")), Some(301));
    }

    #[test]
    fn challenge_part_two() {
        assert_eq!(
            part_two(include_str!("../challenge.txt")),
            Some(3558714869436)
        );
    }
}
