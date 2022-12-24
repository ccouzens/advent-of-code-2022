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
    id: u32,
    costs: [[u32; 4]; 4],
    max_robots_required: [u32; 4],
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
                    max_robots_required: [
                        ore_robot_cost_ore.max(clay_robot_cost_ore).max(obsidian_robot_cost_ore).max(geode_robot_cost_ore),
                        obsidian_robot_cost_clay,
                        geode_robot_cost_obsidian,
                        50
                    ]
                }
            },
        )(input)
    }

    fn geode_count(&self, rounds: u8) -> u32 {
        let start = World {
            robot_counts: [1, 0, 0, 0],
            resource_counts: [0, 0, 0, 0],
            blueprint: self,
        };
        let mut next_possibilities = vec![start];
        for i in 0..rounds {
            #[cfg(feature = "show-progress")]
            if i >= 23 {
                println!(
                    "{} beginning round {}, {}",
                    self.id,
                    i,
                    next_possibilities.len()
                );
            }
            let mut possibilities = HashSet::<World>::new();

            if i <= rounds - 2 {
                next_possibilities.sort_by_cached_key(|p| {
                    p.robot_counts.iter().sum::<u32>() + p.resource_counts.iter().sum::<u32>()
                });
                next_possibilities = zip(
                    (0..next_possibilities.len()).rev(),
                    next_possibilities.iter(),
                )
                .filter_map(|(i, p)| {
                    (i > 10000
                        || !next_possibilities.iter().rev().take(i).take(5000).any(|o| {
                            zip(p.robot_counts.iter(), o.robot_counts.iter())
                                .all(|(p_rc, o_rc)| p_rc <= o_rc)
                                && zip(p.resource_counts.iter(), o.resource_counts.iter())
                                    .all(|(p_rc, o_rc)| p_rc <= o_rc)
                        }))
                    .then_some(*p)
                })
                .collect();
                #[cfg(feature = "show-progress")]
                if i >= 23 {
                    println!(
                        "{} second part of round {}",
                        self.id,
                        next_possibilities.len()
                    );
                }
            }
            for possibility in take(&mut next_possibilities).iter() {
                if i >= rounds - 2 {
                    if let Some(possibility) = possibility.build_robot_and_collect(3) {
                        next_possibilities.push(possibility);
                    } else {
                        let possibility = possibility.collect();
                        next_possibilities.push(possibility);
                    }
                } else {
                    if zip(possibility.robot_counts.iter(), self.costs[3].iter())
                        .all(|(&r, &c)| r >= c)
                        && zip(possibility.resource_counts.iter(), self.costs[3].iter())
                            .all(|(&r, &c)| r >= c)
                    {
                        next_possibilities
                            .push(possibility.build_robot_and_collect(3).unwrap());
                    } else {
                        if !possibility.can_afford_everything() {
                            let possibility = possibility.collect();
                            if possibilities.insert(possibility) {
                                next_possibilities.push(possibility);
                            }
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
    robot_counts: [u32; 4],
    resource_counts: [u32; 4],
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
        let max_required = self.blueprint.max_robots_required.get(robot)?;
        if self.robot_counts.get(robot)? >= max_required {
            return None;
        }
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

pub fn part_one(input: &str) -> u32 {
    let blueprints = blueprints(input);
    blueprints
        .par_iter()
        .map(|bp| bp.id * bp.geode_count(24))
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    let blueprints = blueprints(input);
    blueprints
        .par_iter()
        .take(3)
        .map(|bp| bp.id * bp.geode_count(32))
        .product()
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

    #[test]
    fn example_part_two() {
        assert_eq!(part_two(include_str!("../example.txt")), 56 * 62);
    }

    #[test]
    fn challenge_part_two() {
        assert_eq!(part_two(include_str!("../challenge.txt")), 4672);
    }
}
