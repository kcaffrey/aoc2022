use rayon::iter::{ParallelBridge, ParallelIterator};

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<u32> {
    let starting_state = RobotState {
        ore_robots: 1,
        ..Default::default()
    };
    let quality_sum = input
        .lines()
        .enumerate()
        .par_bridge()
        .map(|(id, line)| (id as u32 + 1, line.into()))
        .map(|(id, blueprint)| {
            id * search_recursive(blueprint, starting_state, 24, 0, [false; 4], [false; 4])
        })
        .sum();
    Some(quality_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let starting_state = RobotState {
        ore_robots: 1,
        ..Default::default()
    };
    let product = input
        .lines()
        .take(3)
        .par_bridge()
        .map(|line| line.into())
        .map(|blueprint| search_recursive(blueprint, starting_state, 32, 0, [false; 4], [false; 4]))
        .product::<u32>();
    Some(product)
}

fn search_recursive(
    blueprint: Blueprint,
    state: RobotState,
    minutes_left: u16,
    best_so_far: u32,
    last_buy: [bool; 4],
    last_affordable: [bool; 4],
) -> u32 {
    if minutes_left == 0 {
        return state.geodes as u32;
    }

    // If we were to build a geode robot every turn until the end, see if we can beat the best so far.
    let max_possible_geodes = (minutes_left * (minutes_left - 1) / 2
        + state.geodes
        + state.geode_robots * minutes_left) as u32;
    if max_possible_geodes < best_so_far {
        return 0;
    }

    // First try buying things.
    let mut max = best_so_far;
    let bots_affordable = state.bots_affordable(blueprint);
    let bot_costs = [
        blueprint.ore_robot,
        blueprint.clay_robot,
        blueprint.obsidian_robot,
        blueprint.geode_robot,
    ];
    let bought_last_turn = last_buy.iter().any(|&b| b);
    for bot_index in 0..4 {
        if bots_affordable[bot_index] {
            if bot_index == 0 && minutes_left.saturating_sub(2) < bot_costs[0].ore {
                continue;
            }
            if last_affordable[bot_index] && !bought_last_turn {
                continue;
            }
            let mut new_bots = [false; 4];
            new_bots[bot_index] = true;
            let new_state = state
                .spend(bot_costs[bot_index])
                .collect()
                .add_bots(new_bots);
            max = max.max(search_recursive(
                blueprint,
                new_state,
                minutes_left - 1,
                max,
                new_bots,
                bots_affordable,
            ));
        }
    }

    // Then try ending the turn without buying anything.
    if bots_affordable.into_iter().any(|b| !b) {
        max = max.max(search_recursive(
            blueprint,
            state.collect(),
            minutes_left - 1,
            max,
            [false; 4],
            bots_affordable,
        ));
    }

    max
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash)]
struct RobotState {
    ore_robots: u16,
    clay_robots: u16,
    obsidian_robots: u16,
    geode_robots: u16,
    resources: Resources,
    geodes: u16,
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash)]
struct Blueprint {
    id: u32,
    ore_robot: Resources,
    clay_robot: Resources,
    obsidian_robot: Resources,
    geode_robot: Resources,
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash)]
struct Resources {
    ore: u16,
    clay: u16,
    obsidian: u16,
}

impl RobotState {
    pub const fn collect(self) -> Self {
        let mut ret = self;
        ret.geodes += self.geode_robots;
        ret.resources.ore += self.ore_robots;
        ret.resources.clay += self.clay_robots;
        ret.resources.obsidian += self.obsidian_robots;
        ret
    }

    pub const fn spend(self, costs: Resources) -> Self {
        let mut ret = self;
        ret.resources.ore -= costs.ore;
        ret.resources.clay -= costs.clay;
        ret.resources.obsidian -= costs.obsidian;
        ret
    }

    pub fn add_bots(self, bots: [bool; 4]) -> Self {
        let mut ret = self;
        match bots.into_iter().position(|buy| buy).unwrap_or(4) {
            0 => ret.ore_robots += 1,
            1 => ret.clay_robots += 1,
            2 => ret.obsidian_robots += 1,
            3 => ret.geode_robots += 1,
            _ => {}
        }
        ret
    }

    pub fn bots_affordable(&self, blueprint: Blueprint) -> [bool; 4] {
        [
            self.resources.is_affordable(blueprint.ore_robot),
            self.resources.is_affordable(blueprint.clay_robot),
            self.resources.is_affordable(blueprint.obsidian_robot),
            self.resources.is_affordable(blueprint.geode_robot),
        ]
    }
}

impl Resources {
    pub const fn is_affordable(&self, costs: Self) -> bool {
        costs.ore <= self.ore && costs.clay <= self.clay && costs.obsidian <= self.obsidian
    }
}

impl From<&str> for Blueprint {
    fn from(value: &str) -> Self {
        let (id, costs) = value.split_once(": ").unwrap();
        let id = id[10..].parse::<u32>().unwrap();
        let mut costs = costs.split('.');
        Self {
            id,
            ore_robot: costs.next().unwrap().into(),
            clay_robot: costs.next().unwrap().into(),
            obsidian_robot: costs.next().unwrap().into(),
            geode_robot: costs.next().unwrap().into(),
        }
    }
}

impl From<&str> for Resources {
    fn from(mut value: &str) -> Self {
        let mut costs = Self::default();
        while let Some(next_digit) = value.find(|ch: char| ch.is_ascii_digit()) {
            let (amount, rest) = value[next_digit..].split_once(' ').unwrap();
            let amount = amount.parse().unwrap();
            let end_of_resource = rest
                .find(|ch: char| ch.is_ascii_whitespace())
                .unwrap_or(rest.len());
            let resource = &rest[..end_of_resource];
            match resource {
                "ore" => costs.ore = amount,
                "clay" => costs.clay = amount,
                "obsidian" => costs.obsidian = amount,
                _ => {}
            }
            value = &rest[end_of_resource..];
        }
        costs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
