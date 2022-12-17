use std::iter::zip;

use nom::{
    branch::alt,
    bytes::complete::take_while_m_n,
    character::complete::{char, digit1, newline},
    combinator::{iterator, map, map_res, ParserIterator},
    multi::separated_list0,
    sequence::{delimited, terminated, tuple},
    IResult,
};
use std::cmp::Ordering::*;

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    List(Vec<Packet>),
    Integer(u8),
}

impl Packet {
    fn parse_integer(input: &str) -> IResult<&str, Self> {
        map_res(digit1, |d| str::parse(d).map(Packet::Integer))(input)
    }

    fn parse_list(input: &str) -> IResult<&str, Self> {
        map(
            delimited(
                char('['),
                separated_list0(char(','), Self::parse),
                char(']'),
            ),
            Packet::List,
        )(input)
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        alt((Self::parse_integer, Self::parse_list))(input)
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::Integer(s), Self::Integer(o)) => s.cmp(o),
            (Self::List(s), Self::List(o)) => zip(s.iter(), o.iter())
                .find_map(|(se, oe)| match se.cmp(oe) {
                    Less => Some(Less),
                    Equal => None,
                    Greater => Some(Greater),
                })
                .unwrap_or_else(|| s.len().cmp(&o.len())),
            (Self::Integer(s), o @ Self::List(_)) => Self::List(vec![Self::Integer(*s)]).cmp(o),
            (s @ Self::List(_), Self::Integer(o)) => s.cmp(&Self::List(vec![Self::Integer(*o)])),
        }
    }
}

#[derive(Debug)]
struct PacketPair {
    left: Packet,
    right: Packet,
}

impl PacketPair {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            tuple((Packet::parse, newline, Packet::parse)),
            |(left, _, right)| PacketPair { left, right },
        )(input)
    }

    fn parser_iterator<'a>(
        input: &'a str,
    ) -> ParserIterator<
        &'a str,
        nom::error::Error<&'a str>,
        impl FnMut(&'a str) -> IResult<&'a str, PacketPair>,
    > {
        iterator(
            input,
            terminated(Self::parse, take_while_m_n(1, 2, |c| c == '\n')),
        )
    }

    fn is_ordered(&self) -> bool {
        self.left < self.right
    }
}

pub fn part_one(input: &str) -> usize {
    let mut iter = PacketPair::parser_iterator(input);
    zip(1.., &mut iter)
        .filter_map(|(i, pair)| pair.is_ordered().then_some(i))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_one() {
        assert_eq!(part_one(include_str!("../example.txt")), 13);
    }

    #[test]
    fn challenge_part_one() {
        assert_eq!(part_one(include_str!("../challenge.txt")), 5882);
    }
}
