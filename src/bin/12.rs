use std::collections::{HashSet, VecDeque};

use tinyvec::ArrayVec;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u32> {
    let (height_map, start, goal) = parse_input(input);
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((0, start));
    visited.insert(start);
    while let Some((d, cur)) = queue.pop_front() {
        if cur == goal {
            return Some(d);
        }
        for n in height_map.neighbors(cur) {
            if visited.insert(n) {
                queue.push_back((d + 1, n));
            }
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let (height_map, _, goal) = parse_input(input);
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((0, goal));
    visited.insert(goal);
    while let Some((d, cur)) = queue.pop_front() {
        if height_map.height_at(cur) == 0 {
            return Some(d);
        }
        for n in height_map.reverse_neighbors(cur) {
            if visited.insert(n) {
                queue.push_back((d + 1, n));
            }
        }
    }
    None
}

fn parse_input(input: &str) -> (Map, Position, Position) {
    let mut start = Position::default();
    let mut goal = start;
    let height_map = Map(input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .map(|(col, &ch)| match ch {
                    b'S' => {
                        start = Position { row, col };
                        0
                    }
                    b'E' => {
                        goal = Position { row, col };
                        25
                    }
                    v => v - b'a',
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>());
    (height_map, start, goal)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct OpenPosition {
    position: Position,
    distance_so_far: usize,
    heuristic: usize,
}

impl Ord for OpenPosition {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.distance_so_far + self.heuristic).cmp(&(other.distance_so_far + other.heuristic))
    }
}

impl PartialOrd for OpenPosition {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Default)]
struct Position {
    row: usize,
    col: usize,
}

#[derive(Debug, Clone)]
struct Map(Vec<Vec<u8>>);

impl Map {
    pub fn neighbors(&self, pos: Position) -> impl Iterator<Item = Position> {
        let mut ret = ArrayVec::<[Position; 4]>::new();
        if pos.row > 0 && self.0[pos.row][pos.col] + 1 >= self.0[pos.row - 1][pos.col] {
            ret.push(Position::new(pos.row - 1, pos.col));
        }
        if pos.col > 0 && self.0[pos.row][pos.col] + 1 >= self.0[pos.row][pos.col - 1] {
            ret.push(Position::new(pos.row, pos.col - 1));
        }
        if pos.row + 1 < self.0.len()
            && self.0[pos.row][pos.col] + 1 >= self.0[pos.row + 1][pos.col]
        {
            ret.push(Position::new(pos.row + 1, pos.col));
        }
        if pos.col + 1 < self.0[0].len()
            && self.0[pos.row][pos.col] + 1 >= self.0[pos.row][pos.col + 1]
        {
            ret.push(Position::new(pos.row, pos.col + 1));
        }
        ret.into_iter()
    }

    pub fn reverse_neighbors(&self, pos: Position) -> impl Iterator<Item = Position> {
        let mut ret = ArrayVec::<[Position; 4]>::new();
        if pos.row > 0 && self.0[pos.row][pos.col] <= self.0[pos.row - 1][pos.col] + 1 {
            ret.push(Position::new(pos.row - 1, pos.col));
        }
        if pos.col > 0 && self.0[pos.row][pos.col] <= self.0[pos.row][pos.col - 1] + 1 {
            ret.push(Position::new(pos.row, pos.col - 1));
        }
        if pos.row + 1 < self.0.len()
            && self.0[pos.row][pos.col] <= self.0[pos.row + 1][pos.col] + 1
        {
            ret.push(Position::new(pos.row + 1, pos.col));
        }
        if pos.col + 1 < self.0[0].len()
            && self.0[pos.row][pos.col] <= self.0[pos.row][pos.col + 1] + 1
        {
            ret.push(Position::new(pos.row, pos.col + 1));
        }
        ret.into_iter()
    }

    pub fn height_at(&self, pos: Position) -> u8 {
        self.0[pos.row][pos.col]
    }
}

impl Position {
    pub const fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(29));
    }
}
