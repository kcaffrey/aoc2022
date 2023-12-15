use std::{collections::HashSet, ops::Sub};

use rayon::iter::{
    IntoParallelIterator, IntoParallelRefIterator, ParallelBridge, ParallelIterator,
};

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
    Some(part_one_helper(input, 2_000_000))
}

fn part_one_helper(input: &str, y: i32) -> u32 {
    let sensors_and_beacons = parse(input);
    let mut sensors_at_y = 0;
    let mut beacons = HashSet::new();
    for (sensor, beacon) in &sensors_and_beacons {
        if sensor.y == y {
            sensors_at_y += 1;
        }
        if beacon.y == y {
            beacons.insert(beacon.x);
        }
    }
    let sum = non_beacon_ranges_at_y(&sensors_and_beacons, y)
        .into_iter()
        .map(Range::len)
        .sum::<i32>() as u32;
    sum - sensors_at_y - beacons.len() as u32
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(part_two_helper(input, (0..4_000_001).into()))
}

fn part_two_helper(input: &str, valid_coords: Range<i32>) -> u64 {
    let sensors = parse(input)
        .into_iter()
        .map(|(sensor, beacon)| (sensor, sensor.manhattan_distance(beacon) as i32))
        .collect::<Vec<_>>();
    sensors
        .par_iter()
        .flat_map(|&(sensor, distance)| {
            (-distance - 1..=distance + 1)
                .flat_map(move |d| {
                    let i = (distance + 1) - d.abs();
                    [
                        Coordinate::new(sensor.x - i, sensor.y + d),
                        Coordinate::new(sensor.x + i, sensor.y + d),
                    ]
                })
                .par_bridge()
        })
        .filter(|&coord| valid_coords.contains(coord.x) && valid_coords.contains(coord.y))
        .find_map_any(|coord| {
            if sensors
                .iter()
                .all(|&(sensor, distance)| sensor.manhattan_distance(coord) > distance as u32)
            {
                Some(coord.x as u64 * 4_000_000 + coord.y as u64)
            } else {
                None
            }
        })
        .unwrap()
    // (valid_coords.start..valid_coords.end)
    //     .into_par_iter()
    //     .find_map_any(|y| {
    //         let ranges = non_beacon_ranges_at_y(&sensors_and_beacons, y);
    //         match ranges.len() {
    //             l if l > 1 || l == 1 && ranges[0].end < valid_coords.end => {
    //                 Some((ranges[0].end as u64) * 4_000_000 + y as u64)
    //             }
    //             1 if ranges[0].start > valid_coords.start => {
    //                 Some((ranges[0].start - 1) as u64 * 4_000_000 + y as u64)
    //             }
    //             _ => None,
    //         }
    //     })
    //     .unwrap()
}

fn parse(input: &str) -> Vec<(Coordinate, Coordinate)> {
    input
        .lines()
        .map(|line| {
            let (sensor, beacon) = line.split_once(':').unwrap();
            (parse_coordinate(sensor), parse_coordinate(beacon))
        })
        .collect()
}

fn non_beacon_ranges_at_y(
    sensors_and_beacons: &[(Coordinate, Coordinate)],
    y: i32,
) -> Vec<Range<i32>> {
    let mut ranges = sensors_and_beacons
        .iter()
        .copied()
        .filter_map(|(sensor, beacon)| {
            let perpendicular_distance = sensor.manhattan_distance(Coordinate::new(sensor.x, y));
            let beacon_distance = sensor.manhattan_distance(beacon);
            if beacon_distance < perpendicular_distance {
                None
            } else {
                let diff = (beacon_distance - perpendicular_distance) as i32;
                Some(((sensor.x - diff)..(sensor.x + diff + 1)).into())
            }
        })
        .collect::<Vec<_>>();
    ranges.sort_unstable_by_key(|r: &Range<_>| (r.start, r.end));
    let mut last = 0;
    for i in 1..ranges.len() {
        match ranges[last].union(ranges[i]) {
            None => {
                last += 1;
                ranges[last] = ranges[i];
            }
            Some(union) => ranges[last] = union,
        }
    }
    ranges.drain(last + 1..);
    ranges
}

fn parse_coordinate(s: &str) -> Coordinate {
    let at_idx = s.find(" at ").unwrap();
    let s = &s[at_idx + 4..];
    let (x, y) = s.split_once(", ").unwrap();
    let (_, x) = x.split_once('=').unwrap();
    let (_, y) = y.split_once('=').unwrap();
    Coordinate::new(x.parse().unwrap(), y.parse().unwrap())
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub const fn manhattan_distance(self, other: Self) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
struct Range<T> {
    start: T,
    end: T,
}

impl<T> Range<T> {
    pub const fn new(start: T, end: T) -> Self {
        Self { start, end }
    }
}

impl<T: Ord + Copy> Range<T> {
    pub fn union(self, other: Self) -> Option<Self> {
        self.overlaps(other)
            .then(|| Self::new(self.start.min(other.start), self.end.max(other.end)))
    }

    pub fn overlaps(self, other: Self) -> bool {
        other.start < self.end && other.end >= self.start
    }

    pub fn contains(self, item: T) -> bool {
        item >= self.start && item < self.end
    }
}

impl<T: Sub<Output = T>> Range<T> {
    pub fn len(self) -> T {
        self.end - self.start
    }
}

impl<T> From<std::ops::Range<T>> for Range<T> {
    fn from(value: std::ops::Range<T>) -> Self {
        Self {
            start: value.start,
            end: value.end,
        }
    }
}

impl<T> From<Range<T>> for std::ops::Range<T> {
    fn from(value: Range<T>) -> Self {
        value.start..value.end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_helper(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, 26);
    }

    #[test]
    fn test_part_two() {
        let result = part_two_helper(
            &advent_of_code::template::read_file("examples", DAY),
            (0..21).into(),
        );
        assert_eq!(result, 56000011);
    }
}
