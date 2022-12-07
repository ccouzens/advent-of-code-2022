use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, newline, not_line_ending},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Debug)]
enum ConsoleLine<'a> {
    Cd(&'a str),
    Ls,
    Directory(&'a str),
    File { name: &'a str, size: u64 },
}

fn parse_cd(input: &str) -> IResult<&str, ConsoleLine> {
    preceded(tag("$ cd "), map(not_line_ending, ConsoleLine::Cd))(input)
}

fn parse_ls(input: &str) -> IResult<&str, ConsoleLine> {
    map(tag("$ ls"), |_| ConsoleLine::Ls)(input)
}

fn parse_directory(input: &str) -> IResult<&str, ConsoleLine> {
    preceded(tag("dir "), map(not_line_ending, ConsoleLine::Directory))(input)
}

fn parse_file(input: &str) -> IResult<&str, ConsoleLine> {
    map(
        tuple((map_res(digit1, str::parse), not_line_ending)),
        |(size, name)| ConsoleLine::File { name, size },
    )(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<ConsoleLine>> {
    separated_list1(
        newline,
        alt((parse_cd, parse_ls, parse_directory, parse_file)),
    )(input)
}

pub fn part_one(input: &str) -> Result<u64, &'static str> {
    let input = parse_input(input).map_err(|_| "Error parsing input")?.1;
    dbg!(input);
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_one() {
        assert_eq!(part_one(include_str!("../example.txt")), Ok(95437));
    }
}
