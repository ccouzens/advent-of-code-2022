use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{digit1, newline},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

use std::{
    collections::{hash_map, HashMap, HashSet},
    mem::take,
};

const START_VALVE: &str = "AA";

#[derive(Debug)]
struct ValveData<'a> {
    name: &'a str,
    flow_rate: u16,
    neighbours: Vec<&'a str>,
}

impl<'a> ValveData<'a> {
    fn parse_nom(input: &'a str) -> IResult<&'a str, ValveData<'a>> {
        let is_valve_name_char: fn(char) -> bool = |c| ('A'..='Z').contains(&c);

        map(
            tuple((
                tag("Valve "),
                take_while(is_valve_name_char),
                tag(" has flow rate="),
                map_res(digit1, str::parse),
                alt((
                    tag("; tunnels lead to valves "),
                    tag("; tunnel leads to valve "),
                )),
                separated_list1(tag(", "), take_while(is_valve_name_char)),
            )),
            |(_, name, _, flow_rate, _, neighbours)| ValveData {
                name,
                flow_rate,
                neighbours,
            },
        )(input)
    }
}

#[derive(Debug)]
struct Valve<'a> {
    flow_rate: u16,
    neighbour_distance: HashMap<&'a str, u16>,
}

#[derive(Debug)]
struct World<'a> {
    valves: HashMap<&'a str, Valve<'a>>,
}

impl<'a> World<'a> {
    fn prepare(input: &'a str) -> Self {
        let valve_datas = separated_list1(newline, ValveData::parse_nom)(input)
            .unwrap()
            .1;
        let mut valves = HashMap::new();

        for vd in valve_datas
            .iter()
            .filter(|&vd| vd.name == START_VALVE || vd.flow_rate > 0)
        {
            let mut neighbour_distance = HashMap::new();
            let mut visited = HashSet::new();
            visited.insert(vd.name);
            let mut recently_visited = vec![vd.name];
            for distance in 1.. {
                if recently_visited.is_empty() {
                    break;
                }
                for &v in take(&mut recently_visited).iter() {
                    if let Some(vd) = valve_datas.iter().find(|vd| vd.name == v) {
                        for &neighbour in vd.neighbours.iter() {
                            if visited.insert(neighbour) {
                                recently_visited.push(neighbour);
                                if valve_datas
                                    .iter()
                                    .any(|vd| vd.name == neighbour && vd.flow_rate > 0)
                                {
                                    neighbour_distance.insert(neighbour, distance);
                                }
                            }
                        }
                    }
                }
            }
            valves.insert(
                vd.name,
                Valve {
                    flow_rate: vd.flow_rate,
                    neighbour_distance,
                },
            );
        }

        World { valves }
    }
}

pub fn part_one(input: &str) -> u16 {
    #[derive(Debug)]
    struct StackItem<'a> {
        name: &'a str,
        neighbour_iter: hash_map::Iter<'a, &'a str, u16>,
        acc_flow: u16,
        time_remaining: u16,
    }

    let world = World::prepare(input);
    let mut best = 0;
    let first = world.valves.get(START_VALVE).unwrap();
    let mut stack = vec![StackItem {
        name: START_VALVE,
        neighbour_iter: first.neighbour_distance.iter(),
        acc_flow: 0,
        time_remaining: 30,
    }];
    while let Some(mut bottom) = stack.pop() {
        let time_remaining = bottom.time_remaining;
        let acc_flow = bottom.acc_flow;

        if let Some(new_stack_item) = bottom
            .neighbour_iter
            .by_ref()
            .filter_map(|(&neighbour_name, &neighbour_distance)| {
                if stack.iter().any(|s| s.name == neighbour_name) {
                    return None;
                }
                let time_remaining = time_remaining.checked_sub(neighbour_distance + 1)?;
                let valve = world.valves.get(neighbour_name)?;
                let acc_flow = acc_flow + time_remaining * valve.flow_rate;
                Some(StackItem {
                    name: neighbour_name,
                    neighbour_iter: valve.neighbour_distance.iter(),
                    acc_flow,
                    time_remaining,
                })
            })
            .next()
        {
            stack.push(bottom);
            best = u16::max(best, new_stack_item.acc_flow);
            stack.push(new_stack_item);
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_one() {
        assert_eq!(part_one(include_str!("../example.txt")), 1651);
    }

    #[test]
    fn challenge_part_one() {
        assert_eq!(part_one(include_str!("../challenge.txt")), 1789);
    }
}
