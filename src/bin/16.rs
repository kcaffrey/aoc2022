use std::{
    collections::HashMap,
    fmt::{Debug, Display, Write},
    num::ParseIntError,
    str::FromStr,
};

use thiserror::Error;

advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<u32> {
    let network = input.parse::<Network>().expect("input should be valid");
    Some(part_one_recursive(
        &network,
        Valve(0),
        30,
        0,
        &mut HashMap::new(),
    ))
}

fn part_one_recursive(
    network: &Network,
    valve: Valve,
    time_remaining: u32,
    closed_valves: u64,
    cache: &mut HashMap<(Valve, u32, u64), u32>,
) -> u32 {
    if time_remaining <= 1 {
        return 0;
    }
    if let Some(&cached) = cache.get(&(valve, time_remaining, closed_valves)) {
        return cached;
    }
    let mut max = 0;
    let flow = network.flow_rates[&valve];
    let valve_mask = 1u64 << network.valve_indexes[&valve];
    if (closed_valves & valve_mask) == 0 && flow > 0 {
        max = (time_remaining - 1) * flow
            + part_one_recursive(
                network,
                valve,
                time_remaining - 1,
                closed_valves | valve_mask,
                cache,
            );
    }
    for &neighbor in &network.adjacency[&valve] {
        max = max.max(part_one_recursive(
            network,
            neighbor,
            time_remaining - 1,
            closed_valves,
            cache,
        ));
    }
    cache.insert((valve, time_remaining, closed_valves), max);
    max
}

#[allow(unused)]
pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[derive(Debug, Clone)]
struct Network {
    adjacency: HashMap<Valve, Vec<Valve>>,
    flow_rates: HashMap<Valve, u32>,
    valve_indexes: HashMap<Valve, u8>,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Valve(u16);

#[derive(Error, Debug)]
enum ParseNetworkError {
    #[error("could not parse valve line")]
    GenericParseError,

    #[error("invalid valve id")]
    InvalidValveId(#[from] ParseValveError),

    #[error("invalid flow rate")]
    InvalidFlowRate(#[from] ParseIntError),
}

impl FromStr for Network {
    type Err = ParseNetworkError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut adjacency = HashMap::new();
        let mut flow_rates = HashMap::new();
        let mut valve_indexes = HashMap::new();
        for (line_idx, line) in s.lines().enumerate() {
            let (valve, tunnels) = line
                .split_once("; ")
                .ok_or(ParseNetworkError::GenericParseError)?;
            let (valve_id, flow_rate) = valve
                .split_once('=')
                .ok_or(ParseNetworkError::GenericParseError)?;
            let valve_id = valve_id[6..=7].parse()?;
            let flow_rate = flow_rate.parse()?;
            let idx = tunnels
                .find(|ch: char| ch.is_ascii_uppercase())
                .ok_or(ParseNetworkError::GenericParseError)?;
            let tunnels = tunnels[idx..]
                .split(", ")
                .map(|s| s.parse())
                .collect::<Result<_, _>>()?;
            adjacency.insert(valve_id, tunnels);
            flow_rates.insert(valve_id, flow_rate);
            valve_indexes.insert(valve_id, line_idx as u8);
        }
        Ok(Self {
            adjacency,
            flow_rates,
            valve_indexes,
        })
    }
}

#[derive(Error, Debug)]
enum ParseValveError {
    #[error("invalid valve id character: {0}")]
    InvalidCharacter(char),

    #[error("unexpected empty valve id string")]
    UnexpectedEmptyString,
}

impl FromStr for Valve {
    type Err = ParseValveError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ParseValveError::UnexpectedEmptyString);
        }
        let mut id = 0;
        for &ch in s.as_bytes() {
            if !ch.is_ascii_uppercase() {
                return Err(ParseValveError::InvalidCharacter(ch as char));
            }
            id = id * 26 + (ch - b'A') as u16;
        }

        Ok(Self(id))
    }
}

impl Debug for Valve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <&Valve as std::fmt::Display>::fmt(&self, f)
    }
}

impl Display for Valve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(char::from((self.0 / 26) as u8 + b'A'))?;
        f.write_char(char::from((self.0 % 26) as u8 + b'A'))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1651));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1707));
    }
}
