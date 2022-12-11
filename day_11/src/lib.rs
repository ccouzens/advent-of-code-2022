use std::{collections::VecDeque, mem::take};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, line_ending},
    combinator::{map, map_res, value},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

#[derive(Debug, Clone, Copy)]
enum Operand {
    Constant(u64),
    Old,
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Mul,
    Add,
}

use Operand::*;
use Operator::*;

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    operation_operator: Operator,
    operation_operand: Operand,
    test_divisor: u64,
    test_true_monkey: usize,
    test_false_monkey: usize,
    inspections: usize,
}

impl Monkey {
    fn parse_nom(input: &str) -> IResult<&str, Monkey> {
        map(
            tuple((
                tag("Monkey "),
                digit1,
                tag(":\n  Starting items: "),
                separated_list1(tag(", "), map_res(digit1, str::parse)),
                tag("\n  Operation: new = old "),
                alt((value(Mul, char('*')), value(Add, char('+')))),
                char(' '),
                alt((
                    value(Old, tag("old")),
                    map(map_res(digit1, str::parse), Constant),
                )),
                tag("\n  Test: divisible by "),
                map_res(digit1, str::parse),
                tag("\n    If true: throw to monkey "),
                map_res(digit1, str::parse),
                tag("\n    If false: throw to monkey "),
                map_res(digit1, str::parse),
            )),
            |(
                _,
                _,
                _,
                items,
                _,
                operation_operator,
                _,
                operation_operand,
                _,
                test_divisor,
                _,
                test_true_monkey,
                _,
                test_false_monkey,
            )| Monkey {
                items: VecDeque::from(items),
                operation_operand,
                operation_operator,
                test_divisor,
                test_false_monkey,
                test_true_monkey,
                inspections: 0,
            },
        )(input)
    }
}

#[derive(Debug)]
struct Monkeys {
    troop: Vec<Monkey>,
    common_divisor: u64,
}

impl Monkeys {
    fn parse_all(input: &str) -> Self {
        let troop = separated_list1(tuple((line_ending, line_ending)), Monkey::parse_nom)(input)
            .unwrap()
            .1;
        Self {
            common_divisor: troop.iter().map(|m| m.test_divisor).product(),
            troop,
        }
    }

    fn round(&mut self, divider: u64) {
        for i in 0..self.troop.len() {
            let monkey = &mut self.troop[i];
            let mut items = take(&mut monkey.items);
            monkey.inspections += items.len();
            let &mut Monkey {
                operation_operand,
                operation_operator,
                test_divisor,
                test_false_monkey,
                test_true_monkey,
                ..
            } = monkey;
            for mut item in items.drain(..) {
                item = match (operation_operator, operation_operand) {
                    (Mul, Constant(c)) => item * c,
                    (Mul, Old) => item * item,
                    (Add, Constant(c)) => item + c,
                    (Add, Old) => item + item,
                } / divider
                    % self.common_divisor;
                self.troop[if item % test_divisor == 0 {
                    test_true_monkey
                } else {
                    test_false_monkey
                }]
                .items
                .push_back(item);
            }
        }
    }
}

fn monkey_business(input: &str, rounds: u16, divisor: u64) -> usize {
    let mut monkeys = Monkeys::parse_all(input);
    for _ in 0..rounds {
        monkeys.round(divisor);
    }
    let mut inspections: Vec<_> = monkeys.troop.iter().map(|m| m.inspections).collect();
    inspections.sort();
    inspections.iter().rev().take(2).product()
}

pub fn part_one(input: &str) -> usize {
    monkey_business(input, 20, 3)
}

pub fn part_two(input: &str) -> usize {
    monkey_business(input, 10000, 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_one() {
        assert_eq!(part_one(include_str!("../example.txt")), 10605);
    }

    #[test]
    fn challenge_part_one() {
        assert_eq!(part_one(include_str!("../challenge.txt")), 100345);
    }

    #[test]
    fn example_part_two() {
        assert_eq!(part_two(include_str!("../example.txt")), 2713310158);
    }

    #[test]
    fn challenge_part_two() {
        assert_eq!(part_two(include_str!("../challenge.txt")), 28537348205);
    }
}
