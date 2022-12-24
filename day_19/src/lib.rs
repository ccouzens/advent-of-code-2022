use std::{collections::HashSet, iter::zip, mem::take};

use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, multispace1},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};
use rayon::prelude::*;

#[derive(Debug, Hash, Eq, PartialEq)]
struct Blueprint {
    id: u16,
    costs: [[u16; 4]; 4],
}

impl Blueprint {
    fn parse_nom(input: &str) -> IResult<&str, Blueprint> {
        map(
            tuple((
                tag("Blueprint "),
                map_res(digit1, str::parse),
                char(':'),
                multispace1,
                tag("Each ore robot costs "),
                map_res(digit1, str::parse),
                tag(" ore."),
                multispace1,
                tag("Each clay robot costs "),
                map_res(digit1, str::parse),
                tag(" ore."),
                multispace1,
                tag("Each obsidian robot costs "),
                map_res(digit1, str::parse),
                tag(" ore and "),
                map_res(digit1, str::parse),
                tag(" clay."),
                multispace1,
                tuple((
                    tag("Each geode robot costs "),
                    map_res(digit1, str::parse),
                    tag(" ore and "),
                    map_res(digit1, str::parse),
                    tag(" obsidian."),
                )),
            )),
            |(
                _,
                id,
                _,
                _,
                _,
                ore_robot_cost_ore,
                _,
                _,
                _,
                clay_robot_cost_ore,
                _,
                _,
                _,
                obsidian_robot_cost_ore,
                _,
                obsidian_robot_cost_clay,
                _,
                _,
                (_, geode_robot_cost_ore, _, geode_robot_cost_obsidian, _),
            )| {
                Blueprint {
                    id,
                    costs: [
                        [ore_robot_cost_ore, 0, 0, 0],
                        [clay_robot_cost_ore, 0, 0, 0],
                        [obsidian_robot_cost_ore, obsidian_robot_cost_clay, 0, 0],
                        [geode_robot_cost_ore, 0, geode_robot_cost_obsidian, 0],
                    ],
                }
            },
        )(input)
    }

    fn geode_count(&self) -> u16 {
        let start = World {
            robot_counts: [1, 0, 0, 0],
            resource_counts: [0, 0, 0, 0],
            blueprint: self,
        };
        let mut next_possibilities = vec![start];
        for _ in 0..24 {
            let mut possibilities = HashSet::<World>::new();

            for possibility in take(&mut next_possibilities).iter() {
                if !possibility.can_afford_everything()
                    && possibilities.insert(possibility.collect())
                {
                    next_possibilities.push(possibility.collect());
                }
                for robot in 0..4 {
                    if let Some(possibility) = possibility.build_robot_and_collect(robot) {
                        if possibilities.insert(possibility) {
                            next_possibilities.push(possibility);
                        }
                    }
                }
            }
        }
        next_possibilities
            .iter()
            .map(|p| p.resource_counts[3])
            .max()
            .unwrap_or(0)
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
struct World<'a> {
    robot_counts: [u16; 4],
    resource_counts: [u16; 4],
    blueprint: &'a Blueprint,
}

impl<'a> World<'a> {
    fn collect(&self) -> Self {
        let mut world = *self;
        for (resource, robot) in zip(world.resource_counts.iter_mut(), world.robot_counts.iter()) {
            *resource += *robot;
        }
        world
    }

    fn can_afford_everything(&self) -> bool {
        self.blueprint.costs.iter().all(|costs| {
            zip(costs.iter(), self.resource_counts.iter())
                .all(|(cost, available)| cost <= available)
        })
    }

    fn build_robot_and_collect(&self, robot: usize) -> Option<Self> {
        let mut world = *self;
        let costs = self.blueprint.costs.get(robot)?;
        for (resource, cost) in zip(world.resource_counts.iter_mut(), costs.iter()) {
            *resource = resource.checked_sub(*cost)?;
        }
        for (resource, robot) in zip(world.resource_counts.iter_mut(), world.robot_counts.iter()) {
            *resource += *robot;
        }
        *world.robot_counts.get_mut(robot)? += 1;
        Some(world)
    }
}

fn blueprints(input: &str) -> Vec<Blueprint> {
    separated_list1(multispace1, Blueprint::parse_nom)(input)
        .unwrap()
        .1
}

pub fn part_one(input: &str) -> u16 {
    let blueprints = blueprints(input);
    blueprints
        .par_iter()
        .map(|bp| bp.id * bp.geode_count())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_one() {
        assert_eq!(part_one(include_str!("../example.txt")), 33);
    }

    #[test]
    fn challenge_part_one() {
        assert_eq!(part_one(include_str!("../challenge.txt")), 1962);
    }
}